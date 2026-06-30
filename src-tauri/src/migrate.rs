use crate::tool_map::{get_tool_map, MigrationStrategy};
use crate::path_util::resolve_path;
use crate::check;
use crate::logger::MigrationLogger;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationRecord {
    pub tool_id: String,
    pub tool_name: String,
    pub original_paths: Vec<String>,
    pub target_path: String,
    pub strategy: String,
    pub timestamp: String,
    pub env_vars: Vec<String>,
    pub file_count: u64,
    pub total_size: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct MigrationProgress {
    pub tool_id: String,
    pub tool_name: String,
    pub step: String,
    pub progress: u8,
    pub message: String,
    pub speed: String,
    pub eta: String,
}

impl MigrationProgress {
    fn new(tool_id: &str, tool_name: &str, step: &str, progress: u8, message: &str) -> Self {
        Self {
            tool_id: tool_id.to_string(),
            tool_name: tool_name.to_string(),
            step: step.to_string(),
            progress,
            message: message.to_string(),
            speed: String::new(),
            eta: String::new(),
        }
    }
}

/// 执行迁移
pub fn migrate_tool(
    tool_id: &str,
    target_base: &str,
    on_progress: impl Fn(MigrationProgress),
) -> Result<MigrationRecord, String> {
    let tool_map = get_tool_map();
    let tool = tool_map.get(tool_id).ok_or("工具不存在")?;
    let logger = MigrationLogger::new();
    let start_time = Instant::now();

    logger.info(&format!("开始迁移工具: {}", tool.name));
    on_progress(MigrationProgress::new(tool_id, &tool.name, "checking", 10, "检查前置条件..."));

    // 1. 检查进程占用
    for proc_name in &tool.processes {
        if check::is_process_running(proc_name) {
            return Err(format!("{} 正在运行，请先关闭后再迁移", tool.name));
        }
    }

    // 2. 计算总大小 + 检查磁盘空间
    let mut total_size = 0u64;
    for path_template in &tool.paths {
        let resolved = resolve_path(path_template);
        if resolved.exists() {
            total_size += crate::path_util::get_dir_size(&resolved);
        }
    }

    let free_space = check::get_disk_free_space(target_base);
    if free_space < total_size + 1024 * 1024 * 100 {
        return Err(format!(
            "目标磁盘空间不足，需要 {}，可用 {}",
            crate::path_util::format_size(total_size),
            crate::path_util::format_size(free_space)
        ));
    }

    on_progress(MigrationProgress::new(tool_id, &tool.name, "copying", 20, "准备复制文件..."));

    // 3. 根据策略执行迁移
    let target_dir = PathBuf::from(target_base).join("c-drive-cleaner").join(tool_id);
    fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;

    let mut migrated_paths = Vec::new();
    let mut total_file_count = 0u64;

    // 用于失败回滚的清理操作记录
    let mut created_junctions: Vec<PathBuf> = Vec::new();
    let mut set_env_vars: Vec<String> = Vec::new();

    let result = match tool.strategy {
        MigrationStrategy::Junction => {
            migrate_junction(tool, &target_dir, &mut migrated_paths, &mut total_file_count,
                             &mut created_junctions, &logger, &on_progress, tool_id, start_time, total_size)
        }
        MigrationStrategy::EnvVar => {
            migrate_env_var(tool, &target_dir, &mut migrated_paths, &mut total_file_count,
                            &mut created_junctions, &mut set_env_vars, &logger, &on_progress, tool_id, start_time, total_size)
        }
        MigrationStrategy::VirtualDisk => {
            Err(format!(
                "虚拟化工具不支持自动迁移。\n\n请按照官方流程操作：\n\
                 • Docker Desktop: 设置 → Resources → Disk image location 更改路径\n\
                 • WSL2: 执行 wsl --export <distro> <file> 导出，再 wsl --import 导入到新位置\n\
                 • VirtualBox/VMware: 在虚拟机设置中修改磁盘路径"
            ))
        }
    };

    // 失败自动回滚
    if let Err(ref _e) = result {
        logger.warn(&format!("迁移失败，开始自动回滚..."));
        // 删除已创建的 Junction
        for j in &created_junctions {
            if j.exists() { fs::remove_dir(j).ok(); }
        }
        // 删除已设置的环境变量
        for env_var in &set_env_vars {
            delete_env_var(env_var);
        }
        // 清理目标目录
        if target_dir.exists() { fs::remove_dir_all(&target_dir).ok(); }
        logger.warn("回滚完成");
    }

    let record = result?;

    on_progress(MigrationProgress::new(tool_id, &tool.name, "completed", 100, "迁移完成"));
    logger.info(&format!("工具 {} 迁移完成", tool.name));

    Ok(record)
}

fn migrate_junction(
    tool: &crate::tool_map::ToolDefinition,
    target_dir: &Path,
    migrated_paths: &mut Vec<String>,
    total_file_count: &mut u64,
    created_junctions: &mut Vec<PathBuf>,
    logger: &MigrationLogger,
    on_progress: &impl Fn(MigrationProgress),
    tool_id: &str,
    start_time: Instant,
    total_size: u64,
) -> Result<MigrationRecord, String> {
    let mut total_bytes_copied = 0u64;

    for path_template in &tool.paths {
        let source = resolve_path(path_template);
        if !source.exists() || !source.is_dir() { continue; }

        let dest = target_dir.join(source.file_name().unwrap_or_default());

        on_progress(MigrationProgress::new(tool_id, &tool.name, "copying",
            15, &format!("复制 {} ...", source.display())));

        let (files, _bytes) = copy_dir_all_tracked(&source, &dest, start_time, total_size, on_progress, tool_id, &tool.name, &mut total_bytes_copied)?;
        *total_file_count += files;
        logger.info(&format!("已复制: {} -> {} ({} 文件)", source.display(), dest.display(), files));

        // 校验文件完整性
        let src_size = crate::path_util::get_dir_size(&source);
        let dst_size = crate::path_util::get_dir_size(&dest);
        if dst_size < src_size {
            return Err(format!("文件完整性校验失败: 源 {} 目标 {}", src_size, dst_size));
        }

        // 校验通过，删除原目录以释放 C 盘空间
        fs::remove_dir_all(&source).map_err(|e| format!("删除原目录 {} 失败: {}", source.display(), e))?;
        logger.info(&format!("已删除原目录: {}（C 盘空间已释放）", source.display()));

        // 创建 Junction
        create_junction(&dest, &source)?;
        created_junctions.push(source.clone());
        logger.info(&format!("已创建 Junction: {} -> {}", source.display(), dest.display()));

        migrated_paths.push(path_template.clone());
    }

    let elapsed = start_time.elapsed().as_secs_f64();
    let _speed = if elapsed > 0.0 { total_size as f64 / elapsed } else { 0.0 };

    Ok(MigrationRecord {
        tool_id: tool_id.to_string(),
        tool_name: tool.name.clone(),
        original_paths: migrated_paths.clone(),
        target_path: target_dir.to_string_lossy().to_string(),
        strategy: "Junction".to_string(),
        timestamp: chrono::Local::now().to_rfc3339(),
        env_vars: vec![],
        file_count: *total_file_count,
        total_size,
    })
}

fn migrate_env_var(
    tool: &crate::tool_map::ToolDefinition,
    target_dir: &Path,
    migrated_paths: &mut Vec<String>,
    total_file_count: &mut u64,
    created_junctions: &mut Vec<PathBuf>,
    set_env_vars: &mut Vec<String>,
    logger: &MigrationLogger,
    on_progress: &impl Fn(MigrationProgress),
    tool_id: &str,
    start_time: Instant,
    total_size: u64,
) -> Result<MigrationRecord, String> {
    let mut total_bytes_copied = 0u64;

    for path_template in &tool.paths {
        let source = resolve_path(path_template);
        if !source.exists() || !source.is_dir() { continue; }

        let dest = target_dir.join(source.file_name().unwrap_or_default());

        on_progress(MigrationProgress::new(tool_id, &tool.name, "copying",
            15, &format!("复制 {} ...", source.display())));

        let (files, _bytes) = copy_dir_all_tracked(&source, &dest, start_time, total_size, on_progress, tool_id, &tool.name, &mut total_bytes_copied)?;
        *total_file_count += files;
        logger.info(&format!("已复制: {} -> {}", source.display(), dest.display()));
        migrated_paths.push(path_template.clone());
    }

    // 校验
    let dst_size = crate::path_util::get_dir_size(&target_dir.to_path_buf());
    if dst_size < total_size {
        return Err(format!("文件完整性校验失败: 源 {} 目标 {}", total_size, dst_size));
    }

    // 删除原目录以释放 C 盘空间 + 创建 Junction 兼容旧路径
    for path_template in &tool.paths {
        let source = resolve_path(path_template);
        if source.exists() {
            fs::remove_dir_all(&source).map_err(|e| format!("删除原目录 {} 失败: {}", source.display(), e))?;
            logger.info(&format!("已删除原目录: {}（C 盘空间已释放）", source.display()));
        }
        // 创建 Junction 使旧路径透明跳转到目标目录
        let dest = target_dir.join(source.file_name().unwrap_or_default());
        if dest.exists() && !source.exists() {
            create_junction(&dest, &source)?;
            created_junctions.push(source.clone());
            logger.info(&format!("已创建 Junction 兼容: {} -> {}", source.display(), dest.display()));
        }
    }

    on_progress(MigrationProgress::new(tool_id, &tool.name, "env_set", 80, "设置环境变量..."));

    // 设置环境变量
    for env_var in &tool.env_vars {
        let value = target_dir.to_string_lossy().to_string();
        set_user_env(env_var, &value)?;
        set_env_vars.push(env_var.clone());
        logger.info(&format!("已设置环境变量: {}={}", env_var, value));
    }

    Ok(MigrationRecord {
        tool_id: tool_id.to_string(),
        tool_name: tool.name.clone(),
        original_paths: migrated_paths.clone(),
        target_path: target_dir.to_string_lossy().to_string(),
        strategy: "EnvVar".to_string(),
        timestamp: chrono::Local::now().to_rfc3339(),
        env_vars: tool.env_vars.clone(),
        file_count: *total_file_count,
        total_size,
    })
}

/// 递归复制目录（带进度追踪）
fn copy_dir_all_tracked(
    src: &Path, dst: &Path,
    start_time: Instant, total_expected: u64,
    on_progress: &impl Fn(MigrationProgress),
    tool_id: &str, tool_name: &str,
    total_bytes_copied: &mut u64,
) -> Result<(u64, u64), String> {
    fs::create_dir_all(dst).map_err(|e| e.to_string())?;
    let mut file_count = 0u64;
    let mut byte_count = 0u64;

    for entry in fs::read_dir(src).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let ty = entry.file_type().map_err(|e| e.to_string())?;
        let dest_path = dst.join(entry.file_name());

        if ty.is_dir() {
            let (sub_files, sub_bytes) = copy_dir_all_tracked(&entry.path(), &dest_path, start_time, total_expected, on_progress, tool_id, tool_name, total_bytes_copied)?;
            file_count += sub_files;
            byte_count += sub_bytes;
        } else {
            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            fs::copy(entry.path(), &dest_path).map_err(|e| e.to_string())?;
            file_count += 1;
            byte_count += size;
            *total_bytes_copied += size;

            // 更新进度、速度和ETA（每复制一个文件更新一次）
            let elapsed = start_time.elapsed().as_secs_f64();
            if elapsed > 0.5 {
                let speed = *total_bytes_copied as f64 / elapsed;
                let remaining = if total_expected > *total_bytes_copied { total_expected - *total_bytes_copied } else { 0 };
                let eta_secs = if speed > 0.0 { remaining as f64 / speed } else { 0.0 };

                let progress_pct = if total_expected > 0 {
                    ((*total_bytes_copied as f64 / total_expected as f64) * 80.0 + 15.0) as u8
                } else {
                    50
                };
                let progress_pct = progress_pct.min(95);

                let speed_str = crate::path_util::format_size(speed as u64) + "/s";
                let eta_str = if eta_secs > 3600.0 {
                    format!("{:.1}小时", eta_secs / 3600.0)
                } else if eta_secs > 60.0 {
                    format!("{:.0}分钟", eta_secs / 60.0)
                } else if eta_secs > 0.0 {
                    format!("{:.0}秒", eta_secs)
                } else {
                    "即将完成".to_string()
                };

                let mut p = MigrationProgress::new(tool_id, tool_name, "copying",
                    progress_pct,
                    &format!("已复制 {} / {}", crate::path_util::format_size(*total_bytes_copied), crate::path_util::format_size(total_expected)));
                p.speed = speed_str;
                p.eta = eta_str;
                on_progress(p);
            }
        }
    }

    Ok((file_count, byte_count))
}

/// 递归复制目录（简单版，用于回滚回迁）
pub fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), String> {
    fs::create_dir_all(dst).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(src).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let ty = entry.file_type().map_err(|e| e.to_string())?;
        let dest_path = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dest_path)?;
        } else {
            fs::copy(entry.path(), &dest_path).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

/// 创建 NTFS Junction
fn create_junction(target: &Path, link: &Path) -> Result<(), String> {
    let target_str = target.to_string_lossy();
    let link_str = link.to_string_lossy();

    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let output = Command::new("cmd")
        .args(&["/C", "mklink", "/J", &link_str, &target_str])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("创建 Junction 失败: {}", stderr));
    }
    Ok(())
}

/// 设置用户环境变量
fn set_user_env(name: &str, value: &str) -> Result<(), String> {
    use winapi::um::winreg::{RegOpenKeyExW, RegSetValueExW, RegCloseKey, HKEY_CURRENT_USER};
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    const KEY_WRITE: u32 = 0x00020006;
    const REG_SZ: u32 = 1;

    let key_path = OsStr::new("Environment").encode_wide().chain(Some(0)).collect::<Vec<_>>();
    let value_name = OsStr::new(name).encode_wide().chain(Some(0)).collect::<Vec<_>>();
    let value_data: Vec<u16> = OsStr::new(value).encode_wide().chain(Some(0)).collect();
    let mut hkey = std::ptr::null_mut();

    let result = unsafe { RegOpenKeyExW(HKEY_CURRENT_USER, key_path.as_ptr(), 0, KEY_WRITE, &mut hkey) };
    if result != 0 { return Err("无法打开注册表".into()); }

    let result = unsafe { RegSetValueExW(hkey, value_name.as_ptr(), 0, REG_SZ, value_data.as_ptr() as *const u8, (value_data.len() * 2) as u32) };
    unsafe { RegCloseKey(hkey) };
    if result != 0 { return Err("设置环境变量失败".into()); }

    broadcast_env_change();
    Ok(())
}

/// 删除用户环境变量（供回滚使用）
pub fn delete_env_var(name: &str) {
    use winapi::um::winreg::{RegOpenKeyExW, RegDeleteValueW, RegCloseKey, HKEY_CURRENT_USER};
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    const KEY_WRITE: u32 = 0x00020006;

    let key_path = OsStr::new("Environment").encode_wide().chain(Some(0)).collect::<Vec<_>>();
    let value_name = OsStr::new(name).encode_wide().chain(Some(0)).collect::<Vec<_>>();
    let mut hkey = std::ptr::null_mut();

    let result = unsafe { RegOpenKeyExW(HKEY_CURRENT_USER, key_path.as_ptr(), 0, KEY_WRITE, &mut hkey) };
    if result == 0 {
        unsafe { RegDeleteValueW(hkey, value_name.as_ptr()) };
        unsafe { RegCloseKey(hkey) };
        broadcast_env_change();
    }
}

fn broadcast_env_change() {
    use winapi::um::winuser::{SendMessageTimeoutW, HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE};
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    let env_str: Vec<u16> = OsStr::new("Environment").encode_wide().chain(Some(0)).collect();
    unsafe {
        SendMessageTimeoutW(HWND_BROADCAST, WM_SETTINGCHANGE, 0, env_str.as_ptr() as isize, SMTO_ABORTIFHUNG, 5000, std::ptr::null_mut());
    }
}

/// 保存迁移记录（SQLite）
pub fn save_migration_record(record: &MigrationRecord) -> Result<(), String> {
    crate::db::save_migration(
        &record.tool_id,
        &record.tool_name,
        &record.original_paths,
        &record.target_path,
        &record.strategy,
        &record.timestamp,
        &record.env_vars,
        record.file_count,
        record.total_size,
    )
}
