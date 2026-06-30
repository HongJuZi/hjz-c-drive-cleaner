use rusqlite::{Connection, params};
use serde_json;
use std::path::PathBuf;

/// 数据库路径: %LOCALAPPDATA%/c-drive-cleaner/data.db
fn db_path() -> PathBuf {
    let base = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    let dir = base.join("c-drive-cleaner");
    std::fs::create_dir_all(&dir).ok();
    dir.join("data.db")
}

/// 打开数据库连接，自动创建表
fn open_db() -> Result<Connection, String> {
    let path = db_path();
    let conn = Connection::open(&path).map_err(|e| format!("打开数据库失败: {}", e))?;
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
        .map_err(|e| format!("设置数据库参数失败: {}", e))?;
    init_tables(&conn)?;
    migrate_schema(&conn).ok(); // 兼容旧库，忽略重复列错误
    Ok(conn)
}

fn init_tables(conn: &Connection) -> Result<(), String> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS migration_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tool_id TEXT NOT NULL UNIQUE,
            tool_name TEXT NOT NULL,
            original_paths TEXT NOT NULL,
            target_path TEXT NOT NULL,
            strategy TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            env_vars TEXT NOT NULL DEFAULT '[]',
            file_count INTEGER NOT NULL DEFAULT 0,
            total_size INTEGER NOT NULL DEFAULT 0,
            migrate_count INTEGER NOT NULL DEFAULT 1
        );

        CREATE TABLE IF NOT EXISTS operation_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT NOT NULL,
            level TEXT NOT NULL,
            message TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS scan_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            scan_time TEXT NOT NULL,
            total_tools INTEGER NOT NULL,
            installed_count INTEGER NOT NULL,
            installed_size INTEGER NOT NULL DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS tool_info (
            tool_id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            category TEXT NOT NULL,
            icon TEXT NOT NULL DEFAULT '',
            paths_json TEXT NOT NULL DEFAULT '[]',
            env_vars_json TEXT NOT NULL DEFAULT '[]',
            strategy TEXT NOT NULL DEFAULT '',
            processes_json TEXT NOT NULL DEFAULT '[]',
            description TEXT NOT NULL DEFAULT '',
            risk_level TEXT NOT NULL DEFAULT 'Low',
            first_seen TEXT,
            last_seen TEXT,
            is_installed INTEGER NOT NULL DEFAULT 0,
            migrate_count INTEGER NOT NULL DEFAULT 0,
            last_migrate_time TEXT,
            last_rollback_time TEXT
        );

        CREATE INDEX IF NOT EXISTS idx_logs_time ON operation_logs(timestamp);
        CREATE INDEX IF NOT EXISTS idx_records_tool ON migration_records(tool_id);
        CREATE INDEX IF NOT EXISTS idx_scan_time ON scan_records(scan_time);
        CREATE INDEX IF NOT EXISTS idx_tool_category ON tool_info(category);
        "
    ).map_err(|e| format!("初始化数据库表失败: {}", e))?;
    Ok(())
}

/// 兼容旧库：添加旧表可能缺少的列
fn migrate_schema(conn: &Connection) -> Result<(), String> {
    // migration_records 增加 migrate_count (旧库没有此列)
    conn.execute_batch("ALTER TABLE migration_records ADD COLUMN migrate_count INTEGER NOT NULL DEFAULT 1;").ok();
    Ok(())
}

// ==================== 工具信息同步 ====================

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ToolStatus {
    pub tool_id: String,
    pub name: String,
    pub category: String,
    pub icon: String,
    pub description: String,
    pub risk_level: String,
    pub strategy: String,
    pub is_installed: bool,
    pub migrate_count: u32,
    pub last_migrate_time: String,
    pub last_rollback_time: String,
}

/// 同步 tool_map 全量数据到 tool_info 表
pub fn sync_tool_info(tools: &[crate::detect::DetectedTool]) -> Result<(), String> {
    let conn = open_db()?;
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    for tool in tools {
        // 检查是否已存在
        let exists: bool = conn.query_row(
            "SELECT COUNT(*) FROM tool_info WHERE tool_id = ?1",
            params![tool.id],
            |row| row.get::<_, i64>(0),
        ).map(|c| c > 0).unwrap_or(false);

        if exists {
            // 更新已存在的记录
            conn.execute(
                "UPDATE tool_info SET
                    is_installed = ?1, last_seen = ?2,
                    name = ?3, category = ?4, icon = ?5,
                    description = ?6, risk_level = ?7, strategy = ?8
                 WHERE tool_id = ?9",
                params![
                    tool.installed as i64, now,
                    tool.name, tool.category, tool.icon,
                    tool.description, tool.risk_level, tool.strategy,
                    tool.id,
                ],
            ).map_err(|e| format!("更新工具信息失败: {}", e))?;
        } else {
            // 插入新记录
            conn.execute(
                "INSERT INTO tool_info
                 (tool_id, name, category, icon, description, risk_level, strategy,
                  first_seen, last_seen, is_installed, migrate_count)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, 0)",
                params![
                    tool.id, tool.name, tool.category, tool.icon,
                    tool.description, tool.risk_level, tool.strategy,
                    now, now, tool.installed as i64,
                ],
            ).map_err(|e| format!("插入工具信息失败: {}", e))?;
        }
    }
    Ok(())
}

/// 获取所有工具的持久化状态
pub fn get_all_tool_status() -> Vec<ToolStatus> {
    let conn = match open_db() {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    let mut stmt = match conn.prepare(
        "SELECT tool_id, name, category, icon, description,
                risk_level, strategy, is_installed, migrate_count,
                COALESCE(last_migrate_time,''), COALESCE(last_rollback_time,'')
         FROM tool_info ORDER BY category, name"
    ) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };
    stmt.query_map([], |row| {
        Ok(ToolStatus {
            tool_id: row.get(0)?,
            name: row.get(1)?,
            category: row.get(2)?,
            icon: row.get(3)?,
            description: row.get(4)?,
            risk_level: row.get(5)?,
            strategy: row.get(6)?,
            is_installed: row.get::<_, i64>(7)? != 0,
            migrate_count: row.get::<_, i64>(8)? as u32,
            last_migrate_time: row.get(9)?,
            last_rollback_time: row.get(10)?,
        })
    }).map(|rows| rows.filter_map(|r| r.ok()).collect())
    .unwrap_or_default()
}

/// 增加工具的迁移计数并记录最后迁移时间
pub fn increment_migrate_count(tool_id: &str) -> Result<(), String> {
    let conn = open_db()?;
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "UPDATE tool_info SET migrate_count = migrate_count + 1, last_migrate_time = ?1 WHERE tool_id = ?2",
        params![now, tool_id],
    ).map_err(|e| format!("更新迁移计数失败: {}", e))?;
    Ok(())
}

/// 记录工具的回滚时间和次数（migrate_count 不减，保留历史）
pub fn mark_rollback(tool_id: &str) -> Result<(), String> {
    let conn = open_db()?;
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "UPDATE tool_info SET is_installed = 0, last_rollback_time = ?1 WHERE tool_id = ?2",
        params![now, tool_id],
    ).map_err(|e| format!("记录回滚失败: {}", e))?;
    Ok(())
}

// ==================== 迁移记录 ====================

pub fn save_migration(
    tool_id: &str, tool_name: &str, original_paths: &[String],
    target_path: &str, strategy: &str, timestamp: &str,
    env_vars: &[String], file_count: u64, total_size: u64,
) -> Result<(), String> {
    let conn = open_db()?;
    let paths_json = serde_json::to_string(original_paths).map_err(|e| e.to_string())?;
    let env_json = serde_json::to_string(env_vars).map_err(|e| e.to_string())?;

    // 使用 INSERT ... ON CONFLICT 自动递增 migrate_count
    conn.execute(
        "INSERT INTO migration_records
         (tool_id, tool_name, original_paths, target_path, strategy, timestamp, env_vars, file_count, total_size, migrate_count)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9,
             COALESCE((SELECT migrate_count + 1 FROM migration_records WHERE tool_id = ?1), 1))
         ON CONFLICT(tool_id) DO UPDATE SET
             tool_name = excluded.tool_name,
             original_paths = excluded.original_paths,
             target_path = excluded.target_path,
             strategy = excluded.strategy,
             timestamp = excluded.timestamp,
             env_vars = excluded.env_vars,
             file_count = excluded.file_count,
             total_size = excluded.total_size,
             migrate_count = migration_records.migrate_count + 1",
        params![tool_id, tool_name, paths_json, target_path, strategy, timestamp, env_json, file_count, total_size],
    ).map_err(|e| format!("保存迁移记录失败: {}", e))?;

    // 同步到 tool_info
    let _ = increment_migrate_count(tool_id);
    Ok(())
}

pub fn load_migration(tool_id: &str) -> Result<crate::migrate::MigrationRecord, String> {
    let conn = open_db()?;
    let mut stmt = conn.prepare(
        "SELECT tool_id, tool_name, original_paths, target_path, strategy, timestamp, env_vars, file_count, total_size
         FROM migration_records WHERE tool_id = ?1"
    ).map_err(|e| e.to_string())?;

    let result = stmt.query_row(params![tool_id], |row| {
        let paths_str: String = row.get(2)?;
        let env_str: String = row.get(6)?;
        Ok(crate::migrate::MigrationRecord {
            tool_id: row.get(0)?,
            tool_name: row.get(1)?,
            original_paths: serde_json::from_str(&paths_str).unwrap_or_default(),
            target_path: row.get(3)?,
            strategy: row.get(4)?,
            timestamp: row.get(5)?,
            env_vars: serde_json::from_str(&env_str).unwrap_or_default(),
            file_count: row.get(7)?,
            total_size: row.get(8)?,
        })
    }).map_err(|e| format!("加载迁移记录失败: {}", e))?;
    Ok(result)
}

pub fn get_migrated_tool_ids() -> Vec<String> {
    let conn = match open_db() {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    let mut stmt = match conn.prepare("SELECT tool_id FROM migration_records ORDER BY timestamp DESC") {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };
    let result: Vec<String> = stmt.query_map([], |row| row.get(0))
        .map(|rows| rows.filter_map(|r| r.ok()).collect())
        .unwrap_or_default();
    result
}

/// 获取已迁移工具的迁移次数
pub fn get_migrate_count(tool_id: &str) -> u32 {
    let conn = match open_db() {
        Ok(c) => c,
        Err(_) => return 0,
    };
    conn.query_row(
        "SELECT migrate_count FROM migration_records WHERE tool_id = ?1",
        params![tool_id],
        |row| row.get::<_, i64>(0),
    ).map(|c| c as u32).unwrap_or(0)
}

pub fn delete_migration(tool_id: &str) -> Result<(), String> {
    let conn = open_db()?;
    conn.execute("DELETE FROM migration_records WHERE tool_id = ?1", params![tool_id])
        .map_err(|e| format!("删除迁移记录失败: {}", e))?;
    Ok(())
}

// ==================== 操作日志 ====================

pub fn add_log(level: &str, message: &str) -> Result<(), String> {
    let conn = open_db()?;
    let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO operation_logs (timestamp, level, message) VALUES (?1, ?2, ?3)",
        params![ts, level, message],
    ).map_err(|e| format!("写入日志失败: {}", e))?;
    Ok(())
}

pub fn get_all_logs() -> String {
    let conn = match open_db() {
        Ok(c) => c,
        Err(_) => return String::new(),
    };
    let mut stmt = match conn.prepare("SELECT timestamp, level, message FROM operation_logs ORDER BY id ASC") {
        Ok(s) => s,
        Err(_) => return String::new(),
    };
    let lines: Vec<String> = stmt.query_map([], |row| {
        let ts: String = row.get(0)?;
        let lv: String = row.get(1)?;
        let msg: String = row.get(2)?;
        Ok(format!("[{}] [{}] {}", ts, lv, msg))
    }).map(|rows| rows.filter_map(|r| r.ok()).collect())
    .unwrap_or_default();
    lines.join("\n")
}

pub fn get_latest_logs(limit: usize) -> String {
    let conn = match open_db() {
        Ok(c) => c,
        Err(_) => return String::new(),
    };
    let mut stmt = match conn.prepare(
        "SELECT timestamp, level, message FROM operation_logs ORDER BY id DESC LIMIT ?1"
    ) {
        Ok(s) => s,
        Err(_) => return String::new(),
    };
    let mut lines: Vec<String> = stmt.query_map(params![limit as i64], |row| {
        let ts: String = row.get(0)?;
        let lv: String = row.get(1)?;
        let msg: String = row.get(2)?;
        Ok(format!("[{}] [{}] {}", ts, lv, msg))
    }).map(|rows| rows.filter_map(|r| r.ok()).collect())
    .unwrap_or_default();
    lines.reverse();
    lines.join("\n")
}

/// 获取回滚相关的日志记录
pub fn get_rollback_logs() -> Vec<String> {
    let conn = match open_db() {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    let mut stmt = match conn.prepare(
        "SELECT timestamp, message FROM operation_logs
         WHERE message LIKE '%回滚%' ORDER BY id DESC LIMIT 50"
    ) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };
    stmt.query_map([], |row| {
        let ts: String = row.get(0)?;
        let msg: String = row.get(1)?;
        Ok(format!("[{}] {}", ts, msg))
    }).map(|rows| rows.filter_map(|r| r.ok()).collect())
    .unwrap_or_default()
}

// ==================== 扫描记录 ====================

pub fn save_scan(total_tools: usize, installed_count: usize, installed_size: u64) -> Result<(), String> {
    let conn = open_db()?;
    let ts = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    conn.execute(
        "INSERT INTO scan_records (scan_time, total_tools, installed_count, installed_size) VALUES (?1, ?2, ?3, ?4)",
        params![ts, total_tools as i64, installed_count as i64, installed_size as i64],
    ).map_err(|e| format!("保存扫描记录失败: {}", e))?;
    Ok(())
}
