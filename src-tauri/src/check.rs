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
