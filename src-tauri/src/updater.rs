use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter};
use futures_util::StreamExt;
use sha2::{Sha256, Digest};
use std::io::Write;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub version: String,
    #[serde(default)]
    pub version_code: String,
    pub url: String,
    pub date: String,
    pub hash: String,
    pub author: String,
    pub has_update: bool,
    pub local_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub downloaded: u64,
    pub total: u64,
    pub percent: u8,
}

/// 简单 semver 比较：current < latest 返回 true
fn compare_versions(current: &str, latest: &str) -> bool {
    let parse = |v: &str| -> Vec<u32> {
        v.split('.')
            .filter_map(|s| s.parse::<u32>().ok())
            .collect()
    };
    let cur = parse(current);
    let lat = parse(latest);
    let max_len = cur.len().max(lat.len());
    for i in 0..max_len {
        let c = cur.get(i).copied().unwrap_or(0);
        let l = lat.get(i).copied().unwrap_or(0);
        if l > c {
            return true;
        }
        if l < c {
            return false;
        }
    }
    false
}

/// 获取更新包下载目录（优先非 C 盘）
fn get_update_dir() -> PathBuf {
    // 检查常见非系统盘
    for drive in &["J:\\", "D:\\", "E:\\", "F:\\", "G:\\", "H:\\", "I:\\"] {
        let p = PathBuf::from(format!("{}c-drive-cleaner-updates", drive));
        if p.exists() || std::fs::create_dir_all(&p).is_ok() {
            return p;
        }
    }
    // 回退到 data 目录
    let fallback = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("C:\\Users\\Public"))
        .join("c-drive-cleaner")
        .join("updates");
    std::fs::create_dir_all(&fallback).ok();
    fallback
}

/// 检查更新：请求 API 并对比版本
#[tauri::command]
pub async fn check_update() -> Result<UpdateInfo, String> {
    let url = "http://www.hongjuzi.com.cn/cdrivecleaner/latest";

    let resp = reqwest::get(url)
        .await
        .map_err(|e| format!("检查更新失败: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("服务器返回状态码: {}", resp.status()));
    }

    #[derive(Deserialize)]
    struct RawUpdate {
        version: String,
        #[serde(default)]
        version_code: String,
        url: String,
        date: String,
        hash: String,
        author: String,
    }

    let raw: RawUpdate = resp
        .json()
        .await
        .map_err(|e| format!("解析更新信息失败: {}", e))?;

    let has_update = compare_versions(VERSION, &raw.version);

    Ok(UpdateInfo {
        version: raw.version,
        version_code: raw.version_code,
        url: raw.url,
        date: raw.date,
        hash: raw.hash,
        author: raw.author,
        has_update,
        local_version: VERSION.to_string(),
    })
}

/// 下载更新包，带进度事件
#[tauri::command]
pub async fn download_update(
    app: AppHandle,
    url: String,
) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(600))
        .build()
        .map_err(|e| format!("初始化下载客户端失败: {}", e))?;

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("下载失败: {}", e))?;

    let total = resp.content_length().unwrap_or(0);
    let filename = url
        .split('/')
        .last()
        .unwrap_or("update.zip");
    let update_dir = get_update_dir();
    let filepath = update_dir.join(filename);
    let tmp_path = update_dir.join(format!("{}.download", filename));

    let mut file = std::fs::File::create(&tmp_path)
        .map_err(|e| format!("创建下载文件失败: {}", e))?;
    let mut stream = resp.bytes_stream();
    let mut downloaded = 0u64;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("下载数据失败: {}", e))?;
        file.write_all(&chunk)
            .map_err(|e| format!("写入下载文件失败: {}", e))?;
        downloaded += chunk.len() as u64;
        let percent = if total > 0 {
            ((downloaded as f64 / total as f64) * 100.0) as u8
        } else {
            0
        };
        let _ = app.emit("update:download-progress", DownloadProgress {
            downloaded,
            total,
            percent,
        });
    }

    // 下载完成，重命名
    std::fs::rename(&tmp_path, &filepath)
        .map_err(|e| format!("保存文件失败: {}", e))?;

    Ok(filepath.to_string_lossy().to_string())
}

/// 验证文件 SHA256
#[tauri::command]
pub fn verify_hash(filepath: String, expected_hash: String) -> Result<bool, String> {
    let path = Path::new(&filepath);
    if !path.exists() {
        return Err("文件不存在".into());
    }
    let data = std::fs::read(path)
        .map_err(|e| format!("读取文件失败: {}", e))?;
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let actual = format!("{:x}", hasher.finalize());
    Ok(actual.to_lowercase() == expected_hash.to_lowercase())
}

/// 安装更新：解压 → 查找安装程序 → 运行 → 退出
#[tauri::command]
pub fn install_update(filepath: String) -> Result<(), String> {
    let path = PathBuf::from(&filepath);
    if !path.exists() {
        return Err("更新文件不存在".into());
    }

    // 解压到同级目录
    let base_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("update");
    let extract_dir = path
        .parent()
        .unwrap()
        .join(format!("{}_extracted", base_name));

    // 清空目标目录
    if extract_dir.exists() {
        std::fs::remove_dir_all(&extract_dir).ok();
    }
    std::fs::create_dir_all(&extract_dir)
        .map_err(|e| format!("创建解压目录失败: {}", e))?;

    // 解压 ZIP
    let file = std::fs::File::open(&path)
        .map_err(|e| format!("打开更新文件失败: {}", e))?;
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| format!("读取压缩包失败: {}", e))?;

    for i in 0..archive.len() {
        let mut entry = archive
            .by_index(i)
            .map_err(|e| format!("读取压缩条目失败: {}", e))?;
        let entry_path = extract_dir.join(sanitize_path(entry.name()));
        if entry.is_dir() {
            std::fs::create_dir_all(&entry_path).ok();
        } else {
            if let Some(parent) = entry_path.parent() {
                std::fs::create_dir_all(parent).ok();
            }
            let mut outfile = std::fs::File::create(&entry_path)
                .map_err(|e| format!("创建文件失败: {}", e))?;
            std::io::copy(&mut entry, &mut outfile)
                .map_err(|e| format!("解压失败: {}", e))?;
        }
    }

    // 查找安装程序
    let installer = find_installer(&extract_dir)
        .ok_or_else(|| "未在更新包中找到安装程序".to_string())?;

    // 启动安装程序（静默模式）
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let child = std::process::Command::new(&installer)
        .args(&["/S", "/VERYSILENT", "/SUPPRESSMSGBOXES", "/NORESTART",
                 &format!("/DIR={}", get_install_dir())])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .map_err(|e| format!("启动安装程序失败: {}", e))?;

    // 分离子进程，避免父进程退出影响安装
    drop(child);

    // 优雅退出当前应用
    std::process::exit(0);
}

/// 防止 ZIP 路径穿越
fn sanitize_path(name: &str) -> PathBuf {
    let name = name.replace('\\', "/");
    let clean: String = name
        .split('/')
        .filter(|s| !s.is_empty() && *s != ".." && *s != ".")
        .collect::<Vec<_>>()
        .join("/");
    PathBuf::from(clean)
}

/// 查找解压目录中的安装程序
fn find_installer(dir: &Path) -> Option<PathBuf> {
    let candidates = [
        "setup.exe", "installer.exe", "install.exe",
        "Setup.exe", "Installer.exe", "Install.exe",
        "C盘小清新_Setup.exe", "C盘小清新_installer.exe",
    ];
    for name in &candidates {
        let p = dir.join(name);
        if p.exists() {
            return Some(p);
        }
    }
    // 递归搜索 exe
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(found) = find_installer(&path) {
                    return Some(found);
                }
            } else if path
                .extension()
                .map_or(false, |e| e.to_string_lossy().to_lowercase() == "exe")
            {
                // 跳过 dotnet 运行库等干扰项
                let name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("");
                if !name.contains("dotnet")
                    && !name.contains("vcredist")
                    && !name.contains("DirectX")
                {
                    return Some(path);
                }
            }
        }
    }
    None
}

/// 获取当前安装目录（供安装程序使用）
fn get_install_dir() -> String {
    // 通过当前 exe 路径推断安装目录
    let exe = std::env::current_exe().ok();
    if let Some(p) = exe {
        if let Some(parent) = p.parent() {
            return parent.to_string_lossy().to_string();
        }
    }
    "C:\\Program Files\\C盘小清新".to_string()
}
