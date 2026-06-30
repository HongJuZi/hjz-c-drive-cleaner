use std::path::PathBuf;
use std::env;

/// 解析 Windows 环境变量路径
pub fn resolve_path(path: &str) -> PathBuf {
    let mut resolved = path.to_string();

    // 替换常见环境变量
    if let Ok(userprofile) = env::var("USERPROFILE") {
        resolved = resolved.replace("%USERPROFILE%", &userprofile);
    }
    if let Ok(appdata) = env::var("APPDATA") {
        resolved = resolved.replace("%APPDATA%", &appdata);
    }
    if let Ok(localappdata) = env::var("LOCALAPPDATA") {
        resolved = resolved.replace("%LOCALAPPDATA%", &localappdata);
    }
    if let Ok(programdata) = env::var("PROGRAMDATA") {
        resolved = resolved.replace("%PROGRAMDATA%", &programdata);
    }

    PathBuf::from(resolved)
}

/// 获取目录大小（字节）
pub fn get_dir_size(path: &PathBuf) -> u64 {
    let mut size = 0;
    if path.exists() && path.is_dir() {
        for entry in walkdir::WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.path().is_file() {
                if let Ok(metadata) = entry.metadata() {
                    size += metadata.len();
                }
            }
        }
    }
    size
}

/// 格式化文件大小
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// 检查路径是否为 NTFS Junction
pub fn is_junction(path: &PathBuf) -> bool {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::fs::MetadataExt;
        use winapi::um::winnt::FILE_ATTRIBUTE_REPARSE_POINT;

        if let Ok(metadata) = std::fs::symlink_metadata(path) {
            let attrs = metadata.file_attributes();
            return (attrs & FILE_ATTRIBUTE_REPARSE_POINT) != 0;
        }
    }
    false
}

/// 安全删除 NTFS Junction（使用 cmd /C rmdir 而非 fs::remove_dir）
/// fs::remove_dir 在某些 Windows 版本上无法正确删除 Junction，
/// 可能会误删目标目录内容。cmd /C rmdir 只删除 Junction 本身。
pub fn remove_junction(path: &std::path::Path) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        let path_str = path.to_string_lossy();
        let output = Command::new("cmd")
            .args(&["/C", "rmdir", &path_str])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .map_err(|e| format!("执行 rmdir 失败: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("删除 Junction {} 失败: {}", path.display(), stderr.trim()));
        }
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    {
        std::fs::remove_dir(path).map_err(|e| format!("删除 Junction 失败: {}", e))
    }
}
