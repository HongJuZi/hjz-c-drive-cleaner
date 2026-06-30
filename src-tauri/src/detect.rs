use crate::tool_map::get_tool_map;
use crate::path_util::resolve_path;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DetectedTool {
    pub id: String,
    pub name: String,
    pub category: String,
    pub icon: String,
    pub installed: bool,
    pub size: u64,
    pub size_formatted: String,
    pub paths: Vec<String>,
    pub strategy: String,
    pub description: String,
    pub risk_level: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScanProgress {
    pub tool_name: String,
    pub tool_id: String,
    pub current: usize,
    pub total: usize,
    pub status: String, // "scanning" | "skipped" | "found"
    pub found_count: usize,
    pub found_size: u64,
}

/// 扫描所有工具，返回检测结果（无回调版本，兼容旧调用）
pub fn scan_tools() -> Vec<DetectedTool> {
    scan_tools_with_progress(|_, _, _, _, _, _| {})
}

/// 扫描所有工具，带进度回调
pub fn scan_tools_with_progress<F>(mut on_progress: F) -> Vec<DetectedTool>
where
    F: FnMut(&str, &str, usize, usize, usize, u64), // tool_name, status, current, total, found_count, found_size
{
    let tool_map = get_tool_map();
    let total = tool_map.len();
    let mut results = Vec::new();
    let mut running_found = 0usize;
    let mut running_size = 0u64;

    for (i, (_id, tool)) in tool_map.iter().enumerate() {
        let mut installed = false;
        let mut total_size = 0u64;
        let mut existing_paths = Vec::new();

        // 检查每个路径是否存在
        for path_template in &tool.paths {
            let resolved = resolve_path(path_template);
            if resolved.exists() && resolved.is_dir() {
                installed = true;
                let size = crate::path_util::get_dir_size(&resolved);
                total_size += size;
                existing_paths.push(path_template.clone());
            }
        }

        if installed {
            running_found += 1;
            running_size += total_size;
        }

        let strategy = match tool.strategy {
            crate::tool_map::MigrationStrategy::Junction => "Junction软链接".to_string(),
            crate::tool_map::MigrationStrategy::EnvVar => "环境变量重定向".to_string(),
            crate::tool_map::MigrationStrategy::VirtualDisk => "虚拟磁盘专属".to_string(),
        };

        let risk_level = match tool.risk_level {
            crate::tool_map::RiskLevel::High => "high".to_string(),
            crate::tool_map::RiskLevel::Medium => "medium".to_string(),
            crate::tool_map::RiskLevel::Low => "low".to_string(),
        };

        let status_str = if installed { "found" } else { "skipped" };

        on_progress(&tool.name, status_str, i + 1, total, running_found, running_size);

        results.push(DetectedTool {
            id: tool.id.clone(),
            name: tool.name.clone(),
            category: tool.category.clone(),
            icon: tool.icon.clone(),
            installed,
            size: total_size,
            size_formatted: crate::path_util::format_size(total_size),
            paths: existing_paths,
            strategy,
            description: tool.description.clone(),
            risk_level,
        });
    }

    // 按分类排序
    results.sort_by(|a, b| {
        let category_order = |c: &str| match c {
            "开发工具" => 0,
            "服务器" => 1,
            "编译器" => 2,
            "图形设计软件" => 3,
            "音视频软件" => 4,
            "文档办公软件" => 5,
            "AI 智能体" => 6,
            "其它软件" => 7,
            _ => 8,
        };
        category_order(&a.category).cmp(&category_order(&b.category))
            .then_with(|| a.name.cmp(&b.name))
    });

    results
}
