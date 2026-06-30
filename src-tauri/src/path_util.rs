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
