use std::process::Command;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AdminStatus {
    pub is_admin: bool,
    pub elevated: bool,
}

/// 检查是否以管理员权限运行（使用 Windows API）
pub fn is_admin() -> bool {
    #[cfg(target_os = "windows")]
    {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;

        // 通过尝试创建系统目录下的文件来检测管理员权限
        // 更可靠的方法：检查 token 中的管理员 SID
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        if let Ok(output) = Command::new("whoami")
            .args(&["/groups"])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // 检查输出中是否包含管理员 SID
            return stdout.contains("S-1-5-32-544") && stdout.contains("Enabled");
        }
    }
    false
}

/// 申请管理员权限（通过 UAC 提权重启）
pub fn request_admin() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        use std::env;
        use std::ffi::OsStr;
        use winapi::um::shellapi::ShellExecuteW;
        use winapi::um::winuser::SW_SHOWNORMAL;
        use std::os::windows::ffi::OsStrExt;

        let exe_path = env::current_exe().map_err(|e| e.to_string())?;
        let exe_str = exe_path.to_string_lossy().to_string();

        let wide_path: Vec<u16> = OsStr::new(&exe_str).encode_wide().chain(Some(0)).collect();
        let wide_runas: Vec<u16> = OsStr::new("runas").encode_wide().chain(Some(0)).collect();

        let result = unsafe {
            ShellExecuteW(
                std::ptr::null_mut(),
                wide_runas.as_ptr(),
                wide_path.as_ptr(),
                std::ptr::null(),
                std::ptr::null(),
                SW_SHOWNORMAL,
            )
        };

        if result as isize > 32 {
            Ok(())
        } else {
            Err("用户拒绝管理员权限或提权失败".into())
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("仅支持 Windows 系统".into())
    }
}

/// 检查 Windows 系统版本（仅支持 Win10/11）
pub fn check_windows_version() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;

        // 通过 ver 命令或 RtlGetVersion 获取版本
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        if let Ok(output) = Command::new("cmd")
            .args(&["/C", "ver"])
            .creation_flags(CREATE_NO_WINDOW)
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // Windows 10/11 版本号格式: Microsoft Windows [版本 10.0.xxxxx]
            if stdout.contains("10.0") {
                return Ok("Windows 10/11 已确认".into());
            } else if stdout.contains("6.3") {
                return Err("Windows 8.1 不受支持，请使用 Windows 10 或更高版本".into());
            } else if stdout.contains("6.1") {
                return Err("Windows 7 不受支持，请使用 Windows 10 或更高版本".into());
            }
            return Ok(format!("系统版本: {}", stdout.trim()));
        }

        Err("无法检测系统版本".into())
    }

    #[cfg(not(target_os = "windows"))]
    {
        Err("仅支持 Windows 系统".into())
    }
}
