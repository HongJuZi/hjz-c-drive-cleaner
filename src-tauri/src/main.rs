#![windows_subsystem = "windows"]

mod tool_map;
mod detect;
mod path_util;
mod migrate;
mod rollback;
mod check;
mod admin;
mod logger;
mod db;
mod updater;

use detect::scan_tools;
use migrate::{migrate_tool, save_migration_record};
use rollback::get_migrated_tools;
use serde::Serialize;
use tauri::{command, Emitter, Manager};

#[command]
async fn scan_all_tools(app: tauri::AppHandle) -> Vec<detect::DetectedTool> {
    let results = tokio::task::spawn_blocking(move || {
        detect::scan_tools_with_progress(|name, status, current, total, found_count, found_size| {
            let _ = app.emit("scan-progress", &detect::ScanProgress {
                tool_name: name.to_string(),
                tool_id: String::new(),
                current,
                total,
                status: status.to_string(),
                found_count,
                found_size,
            });
        })
    }).await.unwrap_or_default();

    // 持久化扫描记录
    let installed = results.iter().filter(|t| t.installed).count();
    let total_size: u64 = results.iter().map(|t| t.size).sum();
    let _ = db::save_scan(results.len(), installed, total_size);

    // 同步工具信息到持久化表
    let _ = db::sync_tool_info(&results);

    results
}

#[command]
fn get_all_tool_status() -> Vec<db::ToolStatus> {
    db::get_all_tool_status()
}

#[command]
fn get_rollback_logs() -> Vec<String> {
    db::get_rollback_logs()
}

#[command]
fn get_migrate_count(tool_id: String) -> u32 {
    db::get_migrate_count(&tool_id)
}

#[command]
fn get_migrated_tool_ids() -> Vec<String> {
    get_migrated_tools()
}

/// 获取 C 盘剩余空间（字节）
#[command]
fn get_c_disk_free() -> u64 {
    check::get_disk_free_space("C:\\")
}

/// 获取所有工具分类（去重）
#[command]
fn get_all_categories() -> Vec<String> {
    let tool_map = tool_map::get_tool_map();
    let mut cats: Vec<String> = tool_map.values().map(|t| t.category.clone()).collect::<std::collections::HashSet<_>>().into_iter().collect();
    cats.sort();
    cats
}

/// 获取所有工具定义（不依赖扫描，用于展示支持列表）
#[derive(Debug, Clone, Serialize)]
struct ToolBrief {
    id: String,
    name: String,
    category: String,
    icon: String,
    description: String,
}

#[command]
fn get_all_tool_definitions() -> Vec<ToolBrief> {
    let tool_map = tool_map::get_tool_map();
    let mut tools: Vec<ToolBrief> = tool_map.into_values().map(|t| ToolBrief {
        id: t.id,
        name: t.name,
        category: t.category,
        icon: t.icon,
        description: t.description,
    }).collect();
    tools.sort_by(|a, b| a.category.cmp(&b.category).then(a.name.cmp(&b.name)));
    tools
}

#[command]
async fn migrate_tool_cmd(
    app: tauri::AppHandle,
    tool_id: String,
    target_path: String,
) -> Result<String, String> {
    let migrated = get_migrated_tools();
    if migrated.contains(&tool_id) {
        return Err("该工具已迁移过，请先回滚再重新迁移".into());
    }

    let app2 = app.clone();
    let tool_id2 = tool_id.clone();
    tokio::task::spawn_blocking(move || {
        migrate_tool(&tool_id2, &target_path, |progress| {
            let _ = app2.emit("migration-progress", &progress);
        })
    })
    .await
    .map_err(|e| format!("迁移任务异常: {}", e))?
    .map(|record| {
        save_migration_record(&record)?;
        Ok(format!("{} 迁移成功", record.tool_name))
    })?
}

/// 前置条件检测：返回每个工具的检测结果
#[derive(serde::Serialize)]
struct PreconditionResult {
    tool_id: String,
    tool_name: String,
    risk_level: String,
    processes_running: Vec<String>,
    disk_space_ok: bool,
    disk_free: String,
    disk_needed: String,
    can_migrate: bool,
    warnings: Vec<String>,
}

#[command]
async fn check_preconditions(
    tool_ids: Vec<String>,
    target_path: String,
) -> Vec<PreconditionResult> {
    tokio::task::spawn_blocking(move || {
        let tool_map = tool_map::get_tool_map();
        let free_space = check::get_disk_free_space(&target_path);
        let mut results = Vec::new();

        for tid in &tool_ids {
            if let Some(tool) = tool_map.get(tid) {
                let running_procs: Vec<String> = tool.processes.iter()
                    .filter(|p| check::is_process_running(p))
                    .cloned().collect();

                let mut total_size = 0u64;
                for path_template in &tool.paths {
                    let resolved = path_util::resolve_path(path_template);
                    if resolved.exists() {
                        total_size += path_util::get_dir_size(&resolved);
                    }
                }

                let disk_ok = free_space >= total_size + 1024 * 1024 * 100;
                let risk = match tool.risk_level {
                    tool_map::RiskLevel::High => "high",
                    tool_map::RiskLevel::Medium => "medium",
                    tool_map::RiskLevel::Low => "low",
                };

                let mut warnings = Vec::new();
                if !running_procs.is_empty() {
                    warnings.push(format!("以下进程正在运行: {}", running_procs.join(", ")));
                }
                if !disk_ok {
                    warnings.push(format!("磁盘空间不足: 需要 {}，可用 {}",
                        path_util::format_size(total_size + 1024 * 1024 * 100),
                        path_util::format_size(free_space)));
                }
                if risk == "high" {
                    warnings.push("Junction软链接将修改文件系统结构，操作不可逆（可回滚）".into());
                }
                if risk == "medium" {
                    warnings.push("将修改用户环境变量，影响所有使用该变量的程序".into());
                }

                let can_migrate = running_procs.is_empty() && disk_ok;

                results.push(PreconditionResult {
                    tool_id: tid.clone(),
                    tool_name: tool.name.clone(),
                    risk_level: risk.to_string(),
                    processes_running: running_procs,
                    disk_space_ok: disk_ok,
                    disk_free: path_util::format_size(free_space),
                    disk_needed: path_util::format_size(total_size),
                    can_migrate,
                    warnings,
                });
            }
        }
        results
    }).await.unwrap_or_default()
}

/// 找到空闲空间最多的盘，推荐 DevCache 路径
#[command]
fn get_best_target_path() -> String {
    let drives = ["D:\\", "E:\\", "F:\\", "G:\\", "H:\\", "I:\\", "J:\\", "K:\\", "L:\\", "M:\\", "N:\\", "O:\\", "P:\\", "Q:\\", "R:\\", "S:\\", "T:\\", "U:\\", "V:\\", "W:\\", "X:\\", "Y:\\", "Z:\\"];
    let mut best = "D:\\".to_string();
    let mut best_free = 0u64;

    for drive in &drives {
        let free = check::get_disk_free_space(drive);
        if free > best_free {
            best_free = free;
            best = format!("{}\\DevCache", drive.trim_end_matches('\\'));
        }
    }

    // 保证至少>C盘可用空间
    let c_free = check::get_disk_free_space("C:\\");
    if best_free < c_free {
        best = "D:\\DevCache".to_string();
    }

    best
}

#[command]
async fn rollback_tool_cmd(
    app: tauri::AppHandle,
    tool_id: String,
) -> Result<String, String> {
    // 查询工具名称用于返回消息
    let tool_map = tool_map::get_tool_map();
    let tool_name = tool_map.get(&tool_id).map(|t| t.name.clone()).unwrap_or_else(|| tool_id.clone());

    let app2 = app.clone();
    let tool_id2 = tool_id.clone();
    let result = tokio::task::spawn_blocking(move || {
        rollback::rollback_tool(&tool_id2, |progress| {
            let _ = app2.emit("rollback-progress", &progress);
        })
    })
    .await
    .map_err(|e| format!("回滚任务异常: {}", e))?;

    result.map(|_| format!("{} 回滚成功", tool_name))
}

#[command]
fn get_logs() -> String {
    let l = logger::MigrationLogger::new();
    l.read_latest_log()
}

#[command]
fn get_all_logs() -> String {
    let l = logger::MigrationLogger::new();
    l.read_all_logs()
}

#[command]
fn get_log_dir() -> String {
    let l = logger::MigrationLogger::new();
    l.get_log_dir().to_string_lossy().to_string()
}

#[command]
fn check_admin() -> bool {
    admin::is_admin()
}

#[command]
fn request_admin_cmd() -> Result<(), String> {
    admin::request_admin()
}

/// 在资源管理器中打开任意目录
#[command]
fn open_directory(path: String) -> Result<(), String> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    std::process::Command::new("explorer")
        .arg(&path)
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .map_err(|e| format!("打开文件夹失败: {}", e))?;
    Ok(())
}

#[command]
fn check_system_version() -> Result<String, String> {
    admin::check_windows_version()
}

/// 在资源管理器中打开工具的缓存目录
#[command]
fn open_tool_location(path_template: String) -> Result<(), String> {
    let resolved = path_util::resolve_path(&path_template);
    let path_str = resolved.to_string_lossy().to_string();
    std::process::Command::new("explorer")
        .arg(&path_str)
        .spawn()
        .map_err(|e| format!("打开文件夹失败: {}", e))?;
    Ok(())
}

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
            // 当用户再次启动时，聚焦到已运行的窗口
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            scan_all_tools,
            get_migrated_tool_ids,
            get_c_disk_free,
            get_all_categories,
            migrate_tool_cmd,
            check_preconditions,
            get_best_target_path,
            rollback_tool_cmd,
            get_logs,
            get_all_logs,
            get_log_dir,
            check_admin,
            request_admin_cmd,
            open_directory,
            check_system_version,
            open_tool_location,
            get_all_tool_status,
            get_rollback_logs,
            get_migrate_count,
            get_all_tool_definitions,
            updater::check_update,
            updater::download_update,
            updater::verify_hash,
            updater::install_update,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
