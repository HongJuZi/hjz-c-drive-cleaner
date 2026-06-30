use chrono::Local;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

pub struct MigrationLogger {
    log_file: PathBuf,
    log_dir: PathBuf,
}

impl MigrationLogger {
    pub fn new() -> Self {
        let log_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("c-drive-cleaner")
            .join("logs");
        fs::create_dir_all(&log_dir).ok();

        let log_file = log_dir.join(format!("migration_{}.log", Local::now().format("%Y%m%d_%H%M%S")));
        Self { log_file, log_dir }
    }

    pub fn log(&self, level: &str, message: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let log_line = format!("[{}] [{}] {}\n", timestamp, level, message);

        // 写入文件
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&self.log_file) {
            let _ = file.write_all(log_line.as_bytes());
        }

        // 写入SQLite
        let _ = crate::db::add_log(level, message);
    }

    pub fn info(&self, message: &str) { self.log("INFO", message); }
    pub fn warn(&self, message: &str) { self.log("WARN", message); }
    pub fn error(&self, message: &str) { self.log("ERROR", message); }

    pub fn get_log_dir(&self) -> &PathBuf { &self.log_dir }

    /// 读取最新日志（从SQLite）
    pub fn read_latest_log(&self) -> String {
        crate::db::get_latest_logs(100)
    }

    /// 读取所有日志（从SQLite）
    pub fn read_all_logs(&self) -> String {
        crate::db::get_all_logs()
    }
}
