use std::process::Command;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ProcessInfo {
    pub name: String,
    pub pid: u32,
}

/// 检查进程是否在运行
pub fn is_process_running(process_name: &str) -> bool {
    get_running_processes().iter().any(|p| p.name.to_lowercase() == process_name.to_lowercase())
}

/// 获取所有运行中的进程
pub fn get_running_processes() -> Vec<ProcessInfo> {
    let mut processes = Vec::new();

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        if let Ok(output) = Command::new("tasklist")
            .args(&["/FO", "CSV", "/NH"])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 2 {
                    let name = parts[0].trim_matches('"').to_string();
                    let pid = parts[1].trim_matches('"').parse::<u32>().unwrap_or(0);
                    if !name.is_empty() && pid > 0 {
                        processes.push(ProcessInfo { name, pid });
                    }
                }
            }
        }
    }

    processes
}

/// 检查磁盘空间（字节）
pub fn get_disk_free_space(path: &str) -> u64 {
    #[cfg(target_os = "windows")]
    {
        use std::ffi::OsStr;
        use winapi::um::fileapi::GetDiskFreeSpaceExW;
        use winapi::um::winnt::ULARGE_INTEGER;
        use std::os::windows::ffi::OsStrExt;

        let drive = if path.len() >= 3 { &path[..3] } else { "C:\\" };
        let wide: Vec<u16> = OsStr::new(drive).encode_wide().chain(Some(0)).collect();

        let mut free_bytes_available: ULARGE_INTEGER = unsafe { std::mem::zeroed() };
        let mut total_bytes: ULARGE_INTEGER = unsafe { std::mem::zeroed() };
        let mut total_free_bytes: ULARGE_INTEGER = unsafe { std::mem::zeroed() };

        let result = unsafe {
            GetDiskFreeSpaceExW(
                wide.as_ptr(),
                &mut free_bytes_available,
                &mut total_bytes,
                &mut total_free_bytes,
            )
        };

        if result != 0 {
            unsafe { *total_free_bytes.QuadPart() }
        } else {
            0
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        0
    }
}

/// 检查对源目录的父目录是否有写权限（用于创建 Junction）
pub fn check_parent_write_permission(source_path: &std::path::Path) -> Result<(), String> {
    let parent = source_path.parent()
        .ok_or_else(|| "无法获取父目录".to_string())?;

    if !parent.exists() {
        return Ok(()); // 父目录不存在，不需要检查
    }

    // 尝试在父目录创建一个临时目录来验证写权限
    let test_dir = parent.join(format!(".cdrive_test_{}", std::process::id()));
    match std::fs::create_dir_all(&test_dir) {
        Ok(_) => {
            std::fs::remove_dir(&test_dir).ok();
            Ok(())
        }
        Err(e) => Err(format!("对目录 {} 的父级无写权限: {}", parent.display(), e)),
    }
}

/// 检查源目录本身是否有写权限（用于删除/移动）
pub fn check_dir_write_permission(dir_path: &std::path::Path) -> Result<(), String> {
    if !dir_path.exists() {
        return Ok(());
    }

    // 尝试在目录中创建一个临时文件来验证写权限
    let test_file = dir_path.join(format!(".cdrive_test_{}", std::process::id()));
    match std::fs::write(&test_file, b"test") {
        Ok(_) => {
            std::fs::remove_file(&test_file).ok();
            Ok(())
        }
        Err(e) => Err(format!("对目录 {} 无写权限: {}", dir_path.display(), e)),
    }
}

/// 测试 Junction 创建能力（在临时目录中测试）
pub fn test_junction_capability() -> Result<(), String> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let temp = std::env::temp_dir();
    let test_target = temp.join(format!("cdrive_jtest_target_{}", std::process::id()));
    let test_link = temp.join(format!("cdrive_jtest_link_{}", std::process::id()));

    // 创建测试目标目录
    std::fs::create_dir_all(&test_target)
        .map_err(|e| format!("创建测试目录失败: {}", e))?;

    // 尝试创建 Junction
    let target_str = test_target.to_string_lossy();
    let link_str = test_link.to_string_lossy();

    let result = Command::new("cmd")
        .args(&["/C", "mklink", "/J", &link_str, &target_str])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    // 清理
    if test_link.exists() {
        // Junction 需要用 rmdir 删除，不能用 remove_dir
        Command::new("cmd")
            .args(&["/C", "rmdir", &link_str])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
            .ok();
    }
    std::fs::remove_dir_all(&test_target).ok();

    match result {
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Junction 创建测试失败: {}", stderr.trim()))
        }
        Err(e) => Err(format!("无法执行 mklink 命令: {}", e)),
    }
}
