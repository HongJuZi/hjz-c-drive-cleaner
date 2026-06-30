use crate::migrate::MigrationRecord;
use crate::path_util::resolve_path;
use crate::logger::MigrationLogger;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Debug, Clone, Serialize)]
pub struct RollbackProgress {
    pub tool_id: String,
    pub tool_name: String,
    pub progress: u8,
    pub message: String,
    pub speed: String,
    pub eta: String,
}

impl RollbackProgress {
    fn new(tool_id: &str, tool_name: &str, progress: u8, message: &str) -> Self {
        Self {
            tool_id: tool_id.to_string(),
            tool_name: tool_name.to_string(),
            progress,
            message: message.to_string(),
            speed: String::new(),
            eta: String::new(),
        }
    }
}

/// 回滚工具迁移（带进度回调）
pub fn rollback_tool(
    tool_id: &str,
    on_progress: impl Fn(RollbackProgress),
) -> Result<(), String> {
    let logger = MigrationLogger::new();
    logger.info(&format!("开始回滚工具: {}", tool_id));

    let record = load_migration_record(tool_id)?;
    let total_size = record.total_size;

    on_progress(RollbackProgress::new(tool_id, &record.tool_name, 0, "正在读取迁移记录..."));

    match record.strategy.as_str() {
        "Junction" => {
            for path_template in &record.original_paths {
                let original_path = resolve_path(path_template);
                if crate::path_util::is_junction(&original_path) {
                    // 删除 Junction
                    on_progress(RollbackProgress::new(tool_id, &record.tool_name, 5, "正在删除 Junction 软链接..."));
                    fs::remove_dir(&original_path).map_err(|e| e.to_string())?;
                    logger.info(&format!("已删除 Junction: {}", original_path.display()));

                    // 从目标目录复制回原路径
                    let target = PathBuf::from(&record.target_path);
                    if let Some(file_name) = original_path.file_name() {
                        let src = target.join(file_name);
                        if src.exists() {
                            // 重建原目录的上层路径
                            if let Some(parent) = original_path.parent() {
                                fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                            }
                            on_progress(RollbackProgress::new(tool_id, &record.tool_name, 10,
                                &format!("正在恢复 {} ...", original_path.display())));

                            let start = Instant::now();
                            let mut total_copied = 0u64;
                            let total_expected = if total_size > 0 { total_size } else {
                                crate::path_util::get_dir_size(&src)
                            };

                            copy_dir_all_tracked(&src, &original_path, start, total_expected,
                                &on_progress, tool_id, &record.tool_name, &mut total_copied)?;

                            logger.info(&format!("已从目标目录恢复: {} -> {}", src.display(), original_path.display()));
                        }
                    }
                }
            }
            // 清理目标目录
            on_progress(RollbackProgress::new(tool_id, &record.tool_name, 95, "正在清理目标目录..."));
            let target = PathBuf::from(&record.target_path);
            if target.exists() {
                fs::remove_dir_all(&target).map_err(|e| format!("清理目标目录失败: {}", e))?;
                logger.info(&format!("已清理目标目录: {}", target.display()));
            }
        }
        "EnvVar" => {
            // 先回迁文件到原始路径
            for path_template in &record.original_paths {
                let original_path = resolve_path(path_template);
                let target = PathBuf::from(&record.target_path);

                // 如果是 Junction 则先删除
                if crate::path_util::is_junction(&original_path) {
                    fs::remove_dir(&original_path).map_err(|e| e.to_string())?;
                    logger.info(&format!("已删除 Junction: {}", original_path.display()));
                }

                if let Some(file_name) = original_path.file_name() {
                    let src = target.join(file_name);
                    if src.exists() {
                        if !original_path.exists() {
                            fs::create_dir_all(original_path.parent().unwrap_or(&original_path)).ok();
                        }
                        on_progress(RollbackProgress::new(tool_id, &record.tool_name, 10,
                            &format!("正在恢复 {} ...", original_path.display())));

                        let start = Instant::now();
                        let mut total_copied = 0u64;
                        let total_expected = if total_size > 0 { total_size } else {
                            crate::path_util::get_dir_size(&src)
                        };

                        copy_dir_all_tracked(&src, &original_path, start, total_expected,
                            &on_progress, tool_id, &record.tool_name, &mut total_copied)?;

                        logger.info(&format!("已回迁文件: {} -> {}", src.display(), original_path.display()));
                    }
                }
            }

            // 删除环境变量
            on_progress(RollbackProgress::new(tool_id, &record.tool_name, 90, "正在清理环境变量..."));
            for env_var in &record.env_vars {
                crate::migrate::delete_env_var(env_var);
                logger.info(&format!("已删除环境变量: {}", env_var));
            }

            // 删除目标目录
            on_progress(RollbackProgress::new(tool_id, &record.tool_name, 95, "正在清理目标目录..."));
            let target = PathBuf::from(&record.target_path);
            if target.exists() {
                fs::remove_dir_all(&target).ok();
                logger.info(&format!("已删除目标目录: {}", target.display()));
            }
        }
        "VirtualDisk" => {
            return Err("虚拟化工具不支持自动回滚".into());
        }
        _ => {
            return Err("未知的迁移策略".into());
        }
    }

    on_progress(RollbackProgress::new(tool_id, &record.tool_name, 99, "正在更新数据库记录..."));
    // 从SQLite删除迁移记录
    crate::db::delete_migration(tool_id)?;
    // 持久化回滚状态
    let _ = crate::db::mark_rollback(tool_id);

    on_progress(RollbackProgress::new(tool_id, &record.tool_name, 100, "回滚完成"));
    logger.info(&format!("工具 {} 回滚完成", tool_id));
    Ok(())
}

/// 递归复制目录（带进度追踪，用于回滚回迁）
fn copy_dir_all_tracked(
    src: &Path, dst: &Path,
    start_time: Instant, total_expected: u64,
    on_progress: &impl Fn(RollbackProgress),
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
            let (sub_files, sub_bytes) = copy_dir_all_tracked(
                &entry.path(), &dest_path, start_time, total_expected,
                on_progress, tool_id, tool_name, total_bytes_copied,
            )?;
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
                    ((*total_bytes_copied as f64 / total_expected as f64) * 80.0 + 10.0) as u8
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

                let mut p = RollbackProgress::new(tool_id, tool_name,
                    progress_pct,
                    &format!("已恢复 {} / {}",
                        crate::path_util::format_size(*total_bytes_copied),
                        crate::path_util::format_size(total_expected)));
                p.speed = speed_str;
                p.eta = eta_str;
                on_progress(p);
            }
        }
    }

    Ok((file_count, byte_count))
}

/// 加载迁移记录（SQLite）
fn load_migration_record(tool_id: &str) -> Result<MigrationRecord, String> {
    crate::db::load_migration(tool_id)
}

/// 获取所有已迁移的工具（SQLite）
pub fn get_migrated_tools() -> Vec<String> {
    crate::db::get_migrated_tool_ids()
}
