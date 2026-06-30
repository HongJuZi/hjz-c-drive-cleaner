use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MigrationStrategy {
    Junction,
    EnvVar,
    VirtualDisk,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub id: String,
    pub name: String,
    pub category: String,
    pub icon: String,
    pub paths: Vec<String>,
    pub env_vars: Vec<String>,
    pub strategy: MigrationStrategy,
    pub processes: Vec<String>,
    pub description: String,
    pub risk_level: RiskLevel,
}

pub fn get_tool_map() -> HashMap<String, ToolDefinition> {
    let mut map = HashMap::new();

    map.insert("npm".into(), ToolDefinition {
        id: "npm".into(),
        name: "NPM".into(),
        category: "开发工具".into(),
        icon: "npm".into(),
        paths: vec!["%APPDATA%\\npm".into(), "%LOCALAPPDATA%\\npm-cache".into(), "%USERPROFILE%\\.npm".into()],
        env_vars: vec!["npm_config_cache".into()],
        strategy: MigrationStrategy::EnvVar,
        processes: vec!["node.exe".into()],
        description: "Node.js包管理器 (全局包+缓存)".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("yarn".into(), ToolDefinition {
        id: "yarn".into(),
        name: "Yarn".into(),
        category: "开发工具".into(),
        icon: "yarn".into(),
        paths: vec!["%LOCALAPPDATA%\\Yarn\\Cache".into(), "%USERPROFILE%\\.yarn\\cache".into()],
        env_vars: vec!["YARN_CACHE_FOLDER".into()],
        strategy: MigrationStrategy::EnvVar,
        processes: vec!["node.exe".into()],
        description: "Yarn包管理器 (v1+v2+缓存)".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("pnpm".into(), ToolDefinition {
        id: "pnpm".into(),
        name: "PNPM".into(),
        category: "开发工具".into(),
        icon: "pnpm".into(),
        paths: vec!["%LOCALAPPDATA%\\pnpm\\store".into(), "%APPDATA%\\pnpm".into()],
        env_vars: vec!["PNPM_HOME".into()],
        strategy: MigrationStrategy::EnvVar,
        processes: vec!["node.exe".into()],
        description: "PNPM包管理器 (store+全局包)".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("nvm_windows".into(), ToolDefinition {
        id: "nvm_windows".into(),
        name: "nvm-windows".into(),
        category: "开发工具".into(),
        icon: "nvm".into(),
        paths: vec!["%USERPROFILE%\\.nvm".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["node.exe".into()],
        description: "nvm-windows Node版本管理器".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("pip".into(), ToolDefinition {
        id: "pip".into(),
        name: "Python Pip".into(),
        category: "开发工具".into(),
        icon: "pip".into(),
        paths: vec!["%LOCALAPPDATA%\\pip\\Cache".into()],
        env_vars: vec!["PIP_CACHE_DIR".into()],
        strategy: MigrationStrategy::EnvVar,
        processes: vec!["python.exe".into()],
        description: "Python包管理器 (pip缓存)".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("anaconda".into(), ToolDefinition {
        id: "anaconda".into(),
        name: "Anaconda/Miniconda".into(),
        category: "开发工具".into(),
        icon: "conda".into(),
        paths: vec!["%USERPROFILE%\\.conda\\pkgs".into(), "%USERPROFILE%\\.conda\\envs".into()],
        env_vars: vec!["CONDA_PKGS_DIRS".into()],
        strategy: MigrationStrategy::EnvVar,
        processes: vec!["conda.exe".into(), "python.exe".into()],
        description: "Anaconda/Miniconda环境管理".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("pyenv_win".into(), ToolDefinition {
        id: "pyenv_win".into(),
        name: "pyenv-win".into(),
        category: "开发工具".into(),
        icon: "python".into(),
        paths: vec!["%USERPROFILE%\\.pyenv".into()],
        env_vars: vec!["PYENV_ROOT".into()],
        strategy: MigrationStrategy::EnvVar,
        processes: vec!["python.exe".into()],
        description: "pyenv-win Python版本管理".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("poetry".into(), ToolDefinition {
        id: "poetry".into(),
        name: "Poetry/Pipenv".into(),
        category: "开发工具".into(),
        icon: "python".into(),
        paths: vec!["%LOCALAPPDATA%\\pypoetry\\Cache".into(), "%USERPROFILE%\\.virtualenvs".into()],
        env_vars: vec!["PIPENV_CACHE_DIR".into(), "POETRY_CACHE_DIR".into()],
        strategy: MigrationStrategy::EnvVar,
        processes: vec!["python.exe".into()],
        description: "Poetry/Pipenv虚拟环境缓存".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("maven".into(), ToolDefinition {
        id: "maven".into(),
        name: "Maven".into(),
        category: "开发工具".into(),
        icon: "maven".into(),
        paths: vec!["%USERPROFILE%\\.m2\\repository".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["java.exe".into()],
        description: "Maven本地依赖仓库".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("gradle".into(), ToolDefinition {
        id: "gradle".into(),
        name: "Gradle".into(),
        category: "开发工具".into(),
        icon: "gradle".into(),
        paths: vec!["%USERPROFILE%\\.gradle".into()],
        env_vars: vec!["GRADLE_USER_HOME".into()],
        strategy: MigrationStrategy::EnvVar,
        processes: vec!["java.exe".into()],
        description: "Gradle构建缓存 (全局依赖)".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("android_sdk".into(), ToolDefinition {
        id: "android_sdk".into(),
        name: "Android SDK".into(),
        category: "开发工具".into(),
        icon: "android".into(),
        paths: vec!["%LOCALAPPDATA%\\Android\\Sdk".into()],
        env_vars: vec!["ANDROID_SDK_ROOT".into()],
        strategy: MigrationStrategy::Junction,
        processes: vec!["adb.exe".into()],
        description: "Android SDK开发工具包".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("android_avd".into(), ToolDefinition {
        id: "android_avd".into(),
        name: "Android AVD".into(),
        category: "开发工具".into(),
        icon: "android".into(),
        paths: vec!["%USERPROFILE%\\.android\\avd".into()],
        env_vars: vec!["ANDROID_SDK_HOME".into()],
        strategy: MigrationStrategy::EnvVar,
        processes: vec!["emulator.exe".into()],
        description: "Android AVD模拟器镜像".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("android_studio".into(), ToolDefinition {
        id: "android_studio".into(),
        name: "Android Studio".into(),
        category: "开发工具".into(),
        icon: "android".into(),
        paths: vec!["%APPDATA%\\Google\\AndroidStudio2024".into(), "%LOCALAPPDATA%\\Google\\AndroidStudio2024".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["studio64.exe".into()],
        description: "Android Studio IDE缓存/配置".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("jetbrains".into(), ToolDefinition {
        id: "jetbrains".into(),
        name: "JetBrains全家桶".into(),
        category: "开发工具".into(),
        icon: "jetbrains".into(),
        paths: vec!["%APPDATA%\\JetBrains".into(), "%LOCALAPPDATA%\\JetBrains".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["idea64.exe".into(), "pycharm64.exe".into(), "clion64.exe".into(), "webstorm64.exe".into(), "goland64.exe".into(), "rider64.exe".into()],
        description: "JetBrains全家桶 (IDEA/PyCharm/CLion等) 索引+配置".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("vscode".into(), ToolDefinition {
        id: "vscode".into(),
        name: "VS Code".into(),
        category: "开发工具".into(),
        icon: "vscode".into(),
        paths: vec!["%APPDATA%\\Code".into(), "%USERPROFILE%\\.vscode\\extensions".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["Code.exe".into()],
        description: "Visual Studio Code扩展+配置缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("cursor".into(), ToolDefinition {
        id: "cursor".into(),
        name: "Cursor".into(),
        category: "开发工具".into(),
        icon: "cursor".into(),
        paths: vec!["%APPDATA%\\Cursor".into(), "%LOCALAPPDATA%\\Cursor".into(), "%USERPROFILE%\\.cursor".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["Cursor.exe".into()],
        description: "Cursor AI编辑器缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("flutter".into(), ToolDefinition {
        id: "flutter".into(),
        name: "Flutter/Dart".into(),
        category: "开发工具".into(),
        icon: "flutter".into(),
        paths: vec!["%LOCALAPPDATA%\\Pub\\Cache".into(), "%USERPROFILE%\\.pub".into()],
        env_vars: vec!["PUB_CACHE".into()],
        strategy: MigrationStrategy::EnvVar,
        processes: vec!["dart.exe".into(), "flutter.exe".into()],
        description: "Flutter/Dart包缓存".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("dotnet".into(), ToolDefinition {
        id: "dotnet".into(),
        name: ".NET NuGet".into(),
        category: "开发工具".into(),
        icon: "nuget".into(),
        paths: vec!["%USERPROFILE%\\.nuget\\packages".into()],
        env_vars: vec!["NUGET_PACKAGES".into()],
        strategy: MigrationStrategy::EnvVar,
        processes: vec!["dotnet.exe".into()],
        description: ".NET NuGet包缓存".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("conan".into(), ToolDefinition {
        id: "conan".into(),
        name: "Conan C++".into(),
        category: "开发工具".into(),
        icon: "conan".into(),
        paths: vec!["%USERPROFILE%\\.conan".into()],
        env_vars: vec!["CONAN_USER_HOME".into()],
        strategy: MigrationStrategy::EnvVar,
        processes: vec!["conan.exe".into()],
        description: "Conan C++包管理器缓存".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("git_lfs".into(), ToolDefinition {
        id: "git_lfs".into(),
        name: "Git LFS".into(),
        category: "开发工具".into(),
        icon: "git".into(),
        paths: vec!["%LOCALAPPDATA%\\Git\\lfs".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["git.exe".into()],
        description: "Git LFS大文件存储缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("sourcetree".into(), ToolDefinition {
        id: "sourcetree".into(),
        name: "Sourcetree".into(),
        category: "开发工具".into(),
        icon: "git".into(),
        paths: vec!["%LOCALAPPDATA%\\Atlassian\\SourceTree".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["SourceTree.exe".into()],
        description: "Sourcetree Git客户端缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("gitkraken".into(), ToolDefinition {
        id: "gitkraken".into(),
        name: "GitKraken".into(),
        category: "开发工具".into(),
        icon: "git".into(),
        paths: vec!["%APPDATA%\\GitKraken".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["gitkraken.exe".into()],
        description: "GitKraken Git客户端缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("terraform".into(), ToolDefinition {
        id: "terraform".into(),
        name: "Terraform/Pulumi".into(),
        category: "开发工具".into(),
        icon: "terraform".into(),
        paths: vec!["%USERPROFILE%\\.terraform".into()],
        env_vars: vec!["TF_PLUGIN_CACHE_DIR".into()],
        strategy: MigrationStrategy::EnvVar,
        processes: vec!["terraform.exe".into()],
        description: "Terraform/Pulumi插件缓存".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("postman".into(), ToolDefinition {
        id: "postman".into(),
        name: "Postman/Apifox".into(),
        category: "开发工具".into(),
        icon: "postman".into(),
        paths: vec!["%APPDATA%\\Postman".into(), "%LOCALAPPDATA%\\Postman".into(), "%LOCALAPPDATA%\\Apifox".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["Postman.exe".into()],
        description: "Postman/Apifox API工具缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("insomnia".into(), ToolDefinition {
        id: "insomnia".into(),
        name: "Insomnia".into(),
        category: "开发工具".into(),
        icon: "insomnia".into(),
        paths: vec!["%APPDATA%\\Insomnia".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["Insomnia.exe".into()],
        description: "Insomnia API客户端缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("localstack".into(), ToolDefinition {
        id: "localstack".into(),
        name: "LocalStack".into(),
        category: "开发工具".into(),
        icon: "localstack".into(),
        paths: vec!["%USERPROFILE%\\.localstack".into()],
        env_vars: vec!["LOCALSTACK_VOLUME_DIR".into()],
        strategy: MigrationStrategy::EnvVar,
        processes: vec![],
        description: "LocalStack云模拟资源持久化".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("ollama".into(), ToolDefinition {
        id: "ollama".into(),
        name: "Ollama".into(),
        category: "开发工具".into(),
        icon: "ollama".into(),
        paths: vec!["%USERPROFILE%\\.ollama\\models".into()],
        env_vars: vec!["OLLAMA_MODELS".into()],
        strategy: MigrationStrategy::EnvVar,
        processes: vec!["ollama.exe".into(), "ollama app.exe".into()],
        description: "Ollama本地大模型 (模型文件GB级)".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("lm_studio".into(), ToolDefinition {
        id: "lm_studio".into(),
        name: "LM Studio".into(),
        category: "开发工具".into(),
        icon: "lmstudio".into(),
        paths: vec!["%USERPROFILE%\\.lmstudio\\models".into(), "%APPDATA%\\LM Studio".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["LM Studio.exe".into()],
        description: "LM Studio本地大模型 (模型文件GB级)".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("huggingface".into(), ToolDefinition {
        id: "huggingface".into(),
        name: "Hugging Face".into(),
        category: "开发工具".into(),
        icon: "huggingface".into(),
        paths: vec!["%USERPROFILE%\\.cache\\huggingface".into()],
        env_vars: vec!["HF_HOME".into()],
        strategy: MigrationStrategy::EnvVar,
        processes: vec![],
        description: "Hugging Face模型缓存".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("github_copilot".into(), ToolDefinition {
        id: "github_copilot".into(),
        name: "GitHub Copilot".into(),
        category: "开发工具".into(),
        icon: "copilot".into(),
        paths: vec!["%APPDATA%\\GitHub Copilot".into(), "%LOCALAPPDATA%\\GitHub Copilot".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec![],
        description: "GitHub Copilot AI代码补全缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("unity".into(), ToolDefinition {
        id: "unity".into(),
        name: "Unity".into(),
        category: "开发工具".into(),
        icon: "unity".into(),
        paths: vec!["%LOCALAPPDATA%\\Unity\\cache".into()],
        env_vars: vec!["UPM_CACHE_PATH".into()],
        strategy: MigrationStrategy::EnvVar,
        processes: vec!["Unity.exe".into()],
        description: "Unity游戏引擎缓存".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("unreal".into(), ToolDefinition {
        id: "unreal".into(),
        name: "Unreal Engine".into(),
        category: "开发工具".into(),
        icon: "unreal".into(),
        paths: vec!["%LOCALAPPDATA%\\EpicGames\\UnrealEngine".into(), "%PROGRAMDATA%\\Epic\\UnrealEngineLauncher".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["UnrealEditor.exe".into()],
        description: "虚幻引擎缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("vim".into(), ToolDefinition {
        id: "vim".into(),
        name: "Vim/Neovim".into(),
        category: "开发工具".into(),
        icon: "vim".into(),
        paths: vec!["%USERPROFILE%\\.vim".into(), "%USERPROFILE%\\AppData\\Local\\nvim".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec![],
        description: "Vim/Neovim编辑器配置+插件缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("mysql".into(), ToolDefinition {
        id: "mysql".into(),
        name: "MySQL/MariaDB".into(),
        category: "服务器".into(),
        icon: "mysql".into(),
        paths: vec!["%PROGRAMDATA%\\MySQL\\MySQL8.0\\Data".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["mysqld.exe".into()],
        description: "MySQL/MariaDB数据库数据目录".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("sqlserver".into(), ToolDefinition {
        id: "sqlserver".into(),
        name: "SQL Server".into(),
        category: "服务器".into(),
        icon: "sqlserver".into(),
        paths: vec!["C:\\Program Files\\Microsoft SQL Server\\MSSQL15.MSSQLSERVER\\MSSQL\\DATA".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["sqlservr.exe".into()],
        description: "Microsoft SQL Server数据库文件".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("postgresql".into(), ToolDefinition {
        id: "postgresql".into(),
        name: "PostgreSQL".into(),
        category: "服务器".into(),
        icon: "postgresql".into(),
        paths: vec!["C:\\Program Files\\PostgreSQL\\16\\data".into()],
        env_vars: vec!["PGDATA".into()],
        strategy: MigrationStrategy::EnvVar,
        processes: vec!["postgres.exe".into()],
        description: "PostgreSQL数据库集群".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("redis".into(), ToolDefinition {
        id: "redis".into(),
        name: "Redis".into(),
        category: "服务器".into(),
        icon: "redis".into(),
        paths: vec!["%LOCALAPPDATA%\\Redis".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["redis-server.exe".into()],
        description: "Redis缓存数据库持久化文件".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("elasticsearch".into(), ToolDefinition {
        id: "elasticsearch".into(),
        name: "Elasticsearch".into(),
        category: "服务器".into(),
        icon: "elasticsearch".into(),
        paths: vec!["%PROGRAMDATA%\\Elastic\\Elasticsearch\\data".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["elasticsearch.exe".into()],
        description: "Elasticsearch搜索引擎数据".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("rabbitmq".into(), ToolDefinition {
        id: "rabbitmq".into(),
        name: "RabbitMQ".into(),
        category: "服务器".into(),
        icon: "rabbitmq".into(),
        paths: vec!["%APPDATA%\\RabbitMQ\\db".into()],
        env_vars: vec!["RABBITMQ_BASE".into()],
        strategy: MigrationStrategy::EnvVar,
        processes: vec!["rabbitmq-server.exe".into()],
        description: "RabbitMQ消息队列持久化".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("nginx".into(), ToolDefinition {
        id: "nginx".into(),
        name: "Nginx".into(),
        category: "服务器".into(),
        icon: "nginx".into(),
        paths: vec!["C:\\Program Files\\nginx\\logs".into(), "C:\\Program Files\\nginx\\temp".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["nginx.exe".into()],
        description: "Nginx Web服务器日志+缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("apache".into(), ToolDefinition {
        id: "apache".into(),
        name: "Apache Httpd".into(),
        category: "服务器".into(),
        icon: "apache".into(),
        paths: vec!["C:\\Program Files\\Apache\\logs".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["httpd.exe".into()],
        description: "Apache Httpd服务器日志缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("xampp".into(), ToolDefinition {
        id: "xampp".into(),
        name: "XAMPP/phpStudy".into(),
        category: "服务器".into(),
        icon: "xampp".into(),
        paths: vec!["C:\\xampp\\mysql\\data".into(), "C:\\xampp\\tmp".into(), "C:\\phpstudy_pro\\Extensions\\MySQL5.7\\data".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["apache.exe".into(), "mysqld.exe".into(), "nginx.exe".into()],
        description: "XAMPP/phpStudy本地PHP环境数据".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("docker".into(), ToolDefinition {
        id: "docker".into(),
        name: "Docker Desktop".into(),
        category: "服务器".into(),
        icon: "docker".into(),
        paths: vec!["%LOCALAPPDATA%\\Docker".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::VirtualDisk,
        processes: vec!["Docker Desktop.exe".into(), "com.docker.backend.exe".into()],
        description: "Docker容器平台 (WSL2镜像)".into(),
        risk_level: RiskLevel::Low,
    });

    map.insert("wsl".into(), ToolDefinition {
        id: "wsl".into(),
        name: "WSL2".into(),
        category: "服务器".into(),
        icon: "wsl".into(),
        paths: vec!["%LOCALAPPDATA%\\Packages\\CanonicalGroupLimited".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::VirtualDisk,
        processes: vec!["wsl.exe".into(), "wslhost.exe".into()],
        description: "WSL2 Linux子系统虚拟磁盘".into(),
        risk_level: RiskLevel::Low,
    });

    map.insert("iis".into(), ToolDefinition {
        id: "iis".into(),
        name: "IIS Web服务".into(),
        category: "服务器".into(),
        icon: "iis".into(),
        paths: vec!["C:\\inetpub\\logs".into(), "C:\\inetpub\\wwwroot".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec![],
        description: "Windows IIS Web服务日志+默认站点".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("rust".into(), ToolDefinition {
        id: "rust".into(),
        name: "Rust工具链".into(),
        category: "编译器".into(),
        icon: "rust".into(),
        paths: vec!["%USERPROFILE%\\.rustup".into(), "%USERPROFILE%\\.cargo".into()],
        env_vars: vec!["RUSTUP_HOME".into(), "CARGO_HOME".into()],
        strategy: MigrationStrategy::EnvVar,
        processes: vec!["rustc.exe".into(), "cargo.exe".into()],
        description: "Rust编译工具链 (rustup+cargo缓存)".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("go".into(), ToolDefinition {
        id: "go".into(),
        name: "Go语言".into(),
        category: "编译器".into(),
        icon: "go".into(),
        paths: vec!["%USERPROFILE%\\go\\pkg\\mod".into(), "%LOCALAPPDATA%\\go-build".into()],
        env_vars: vec!["GOPATH".into(), "GOCACHE".into()],
        strategy: MigrationStrategy::EnvVar,
        processes: vec!["go.exe".into()],
        description: "Go语言模块依赖+编译缓存".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("mingw".into(), ToolDefinition {
        id: "mingw".into(),
        name: "MinGW/MSYS2".into(),
        category: "编译器".into(),
        icon: "mingw".into(),
        paths: vec!["%USERPROFILE%\\.cache\\ccache".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["gcc.exe".into(), "g++.exe".into()],
        description: "MinGW/MSYS2编译缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("adobe_ps".into(), ToolDefinition {
        id: "adobe_ps".into(),
        name: "Adobe Photoshop".into(),
        category: "图形设计软件".into(),
        icon: "adobe".into(),
        paths: vec!["%LOCALAPPDATA%\\Adobe\\Photoshop".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["Photoshop.exe".into()],
        description: "Adobe Photoshop暂存盘+缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("adobe_pr".into(), ToolDefinition {
        id: "adobe_pr".into(),
        name: "Adobe Premiere Pro".into(),
        category: "图形设计软件".into(),
        icon: "adobe".into(),
        paths: vec!["%LOCALAPPDATA%\\Adobe\\Premiere Pro".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["Premiere Pro.exe".into()],
        description: "Adobe Premiere Pro媒体缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("adobe_ae".into(), ToolDefinition {
        id: "adobe_ae".into(),
        name: "Adobe After Effects".into(),
        category: "图形设计软件".into(),
        icon: "adobe".into(),
        paths: vec!["%LOCALAPPDATA%\\Adobe\\After Effects".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["AfterFX.exe".into()],
        description: "Adobe After Effects缓存+暂存盘".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("stable_diffusion".into(), ToolDefinition {
        id: "stable_diffusion".into(),
        name: "Stable Diffusion".into(),
        category: "图形设计软件".into(),
        icon: "stable_diffusion".into(),
        paths: vec!["%USERPROFILE%\\.cache\\stable-diffusion".into(), "%LOCALAPPDATA%\\tmp".into()],
        env_vars: vec!["STABLE_DIFFUSION_CACHE".into()],
        strategy: MigrationStrategy::EnvVar,
        processes: vec![],
        description: "Stable Diffusion AI绘图模型缓存".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("blender".into(), ToolDefinition {
        id: "blender".into(),
        name: "Blender".into(),
        category: "图形设计软件".into(),
        icon: "blender".into(),
        paths: vec!["%APPDATA%\\Blender Foundation\\Blender\\cache".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["blender.exe".into()],
        description: "Blender 3D渲染缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("figma".into(), ToolDefinition {
        id: "figma".into(),
        name: "Figma Desktop".into(),
        category: "图形设计软件".into(),
        icon: "figma".into(),
        paths: vec!["%LOCALAPPDATA%\\Figma".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["Figma.exe".into()],
        description: "Figma设计工具离线缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("jianying".into(), ToolDefinition {
        id: "jianying".into(),
        name: "剪映专业版".into(),
        category: "音视频软件".into(),
        icon: "jianying".into(),
        paths: vec!["%LOCALAPPDATA%\\JianyingPro".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["JianyingPro.exe".into()],
        description: "剪映专业版草稿+素材缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("wps".into(), ToolDefinition {
        id: "wps".into(),
        name: "WPS Office".into(),
        category: "文档办公软件".into(),
        icon: "wps".into(),
        paths: vec!["%LOCALAPPDATA%\\Kingsoft\\WPSOffice\\backupcenter".into(), "%USERPROFILE%\\WPSCloudFiles".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["wps.exe".into(), "wpp.exe".into(), "et.exe".into()],
        description: "WPS Office备份+云文档缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("onedrive".into(), ToolDefinition {
        id: "onedrive".into(),
        name: "OneDrive".into(),
        category: "文档办公软件".into(),
        icon: "onedrive".into(),
        paths: vec!["%USERPROFILE%\\OneDrive".into(), "%LOCALAPPDATA%\\Microsoft\\OneDrive".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["OneDrive.exe".into()],
        description: "OneDrive云盘同步缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("baidu_netdisk".into(), ToolDefinition {
        id: "baidu_netdisk".into(),
        name: "百度网盘".into(),
        category: "文档办公软件".into(),
        icon: "baidu".into(),
        paths: vec!["%USERPROFILE%\\BaiduNetdiskDownload".into(), "%LOCALAPPDATA%\\BaiduNetdisk".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["BaiduNetdisk.exe".into()],
        description: "百度网盘PC客户端缓存+下载".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("chrome".into(), ToolDefinition {
        id: "chrome".into(),
        name: "Chrome/Edge浏览器".into(),
        category: "其它软件".into(),
        icon: "chrome".into(),
        paths: vec!["%LOCALAPPDATA%\\Google\\Chrome\\User Data".into(), "%LOCALAPPDATA%\\Microsoft\\Edge\\User Data".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["chrome.exe".into(), "msedge.exe".into()],
        description: "Chrome/Edge浏览器缓存+用户数据".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("claude".into(), ToolDefinition {
        id: "claude".into(),
        name: "Claude桌面端".into(),
        category: "其它软件".into(),
        icon: "claude".into(),
        paths: vec!["%APPDATA%\\Claude".into(), "%LOCALAPPDATA%\\Claude\\Cache".into(), "%USERPROFILE%\\.claude".into()],
        env_vars: vec!["CLAUDE_CONFIG_DIR".into()],
        strategy: MigrationStrategy::Junction,
        processes: vec!["Claude.exe".into()],
        description: "Claude AI桌面端缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("qoderwork".into(), ToolDefinition {
        id: "qoderwork".into(),
        name: "QoderWork".into(),
        category: "其它软件".into(),
        icon: "qoderwork".into(),
        paths: vec!["%APPDATA%\\QoderWork".into(), "%LOCALAPPDATA%\\QoderWork\\model_cache".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["QoderWork.exe".into()],
        description: "QoderWork AI助手缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("virtualbox".into(), ToolDefinition {
        id: "virtualbox".into(),
        name: "VirtualBox/VMware".into(),
        category: "其它软件".into(),
        icon: "virtualbox".into(),
        paths: vec!["%USERPROFILE%\\VirtualBox VMs".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::VirtualDisk,
        processes: vec!["VirtualBox.exe".into(), "vmware.exe".into()],
        description: "虚拟机磁盘 (禁止软链接)".into(),
        risk_level: RiskLevel::Low,
    });

    map.insert("vagrant".into(), ToolDefinition {
        id: "vagrant".into(),
        name: "Vagrant".into(),
        category: "其它软件".into(),
        icon: "vagrant".into(),
        paths: vec!["%USERPROFILE%\\.vagrant.d".into()],
        env_vars: vec!["VAGRANT_HOME".into()],
        strategy: MigrationStrategy::EnvVar,
        processes: vec!["vagrant.exe".into()],
        description: "Vagrant虚拟化镜像缓存".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("hyperv".into(), ToolDefinition {
        id: "hyperv".into(),
        name: "Hyper-V".into(),
        category: "其它软件".into(),
        icon: "hyperv".into(),
        paths: vec!["C:\\Users\\Public\\Documents\\Hyper-V\\Virtual hard disks".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec![],
        description: "Hyper-V虚拟机磁盘默认路径".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("android_emulator".into(), ToolDefinition {
        id: "android_emulator".into(),
        name: "安卓模拟器".into(),
        category: "其它软件".into(),
        icon: "android".into(),
        paths: vec!["%LOCALAPPDATA%\\Microsoft\\EmulatorManager".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec![],
        description: "雷电/MuMu等安卓模拟器镜像数据".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("system_cache".into(), ToolDefinition {
        id: "system_cache".into(),
        name: "系统通用.cache".into(),
        category: "其它软件".into(),
        icon: "cache".into(),
        paths: vec!["%LOCALAPPDATA%\\.cache".into()],
        env_vars: vec!["XDG_CACHE_HOME".into()],
        strategy: MigrationStrategy::EnvVar,
        processes: vec![],
        description: "XDG通用缓存目录".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("local_db".into(), ToolDefinition {
        id: "local_db".into(),
        name: "本地数据库".into(),
        category: "其它软件".into(),
        icon: "db".into(),
        paths: vec!["%LOCALAPPDATA%\\MySQL\\Data".into(), "%LOCALAPPDATA%\\Redis".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["mysqld.exe".into(), "redis-server.exe".into()],
        description: "MySQL/Redis桌面端数据".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("filebeat".into(), ToolDefinition {
        id: "filebeat".into(),
        name: "Filebeat/Logstash".into(),
        category: "其它软件".into(),
        icon: "filebeat".into(),
        paths: vec!["C:\\Program Files\\Filebeat\\data".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["filebeat.exe".into(), "logstash.exe".into()],
        description: "Filebeat/Logstash日志缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("prometheus".into(), ToolDefinition {
        id: "prometheus".into(),
        name: "Prometheus/Grafana".into(),
        category: "其它软件".into(),
        icon: "prometheus".into(),
        paths: vec!["C:\\Program Files\\Prometheus\\data".into()],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["prometheus.exe".into()],
        description: "Prometheus时序数据库+Grafana缓存".into(),
        risk_level: RiskLevel::High,
    });

    // ==================== CLI / AI 开发工具 ====================

    map.insert("kiro".into(), ToolDefinition {
        id: "kiro".into(),
        name: "Kiro".into(),
        category: "开发工具".into(),
        icon: "kiro".into(),
        paths: vec![
            "%APPDATA%\\Kiro".into(),
            "%LOCALAPPDATA%\\Kiro".into(),
            "%USERPROFILE%\\.kiro".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["Kiro.exe".into()],
        description: "AWS AI IDE编辑器缓存与配置".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("antigravity".into(), ToolDefinition {
        id: "antigravity".into(),
        name: "Antigravity IDE".into(),
        category: "开发工具".into(),
        icon: "antigravity".into(),
        paths: vec![
            "%APPDATA%\\Antigravity".into(),
            "%LOCALAPPDATA%\\Antigravity".into(),
            "%USERPROFILE%\\.antigravity".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["Antigravity.exe".into()],
        description: "Antigravity AI IDE缓存与配置".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("zed".into(), ToolDefinition {
        id: "zed".into(),
        name: "Zed".into(),
        category: "开发工具".into(),
        icon: "zed".into(),
        paths: vec![
            "%APPDATA%\\Zed".into(),
            "%LOCALAPPDATA%\\Zed".into(),
            "%USERPROFILE%\\.zed".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["zed.exe".into()],
        description: "Zed高性能编辑器缓存与扩展".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("warp".into(), ToolDefinition {
        id: "warp".into(),
        name: "Warp Terminal".into(),
        category: "开发工具".into(),
        icon: "warp".into(),
        paths: vec![
            "%LOCALAPPDATA%\\WarpTerminal".into(),
            "%APPDATA%\\warp".into(),
            "%USERPROFILE%\\.warp".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["warp.exe".into()],
        description: "Warp AI终端缓存与配置".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("claude_code".into(), ToolDefinition {
        id: "claude_code".into(),
        name: "Claude Code".into(),
        category: "开发工具".into(),
        icon: "claude_code".into(),
        paths: vec![
            "%USERPROFILE%\\.claude".into(),
            "%LOCALAPPDATA%\\Anthropic\\ClaudeCode".into(),
        ],
        env_vars: vec!["CLAUDE_CONFIG_DIR".into()],
        strategy: MigrationStrategy::Junction,
        processes: vec!["claude.exe".into()],
        description: "Claude Code CLI配置与缓存".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("codex".into(), ToolDefinition {
        id: "codex".into(),
        name: "Codex CLI".into(),
        category: "开发工具".into(),
        icon: "codex".into(),
        paths: vec![
            "%USERPROFILE%\\.codex".into(),
            "%LOCALAPPDATA%\\OpenAI\\Codex".into(),
        ],
        env_vars: vec!["CODEX_HOME".into()],
        strategy: MigrationStrategy::Junction,
        processes: vec!["codex.exe".into()],
        description: "OpenAI Codex CLI配置与缓存".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("trae".into(), ToolDefinition {
        id: "trae".into(),
        name: "Trae".into(),
        category: "开发工具".into(),
        icon: "trae".into(),
        paths: vec![
            "%APPDATA%\\Trae".into(),
            "%LOCALAPPDATA%\\Trae".into(),
            "%USERPROFILE%\\.trae".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["Trae.exe".into()],
        description: "字节跳动Trae AI IDE缓存与配置".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("qoder".into(), ToolDefinition {
        id: "qoder".into(),
        name: "Qoder".into(),
        category: "开发工具".into(),
        icon: "qoder".into(),
        paths: vec![
            "%APPDATA%\\Qoder".into(),
            "%LOCALAPPDATA%\\Qoder".into(),
            "%USERPROFILE%\\.qoder".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["Qoder.exe".into()],
        description: "Qoder AI开发工具缓存与配置".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("qwen_code".into(), ToolDefinition {
        id: "qwen_code".into(),
        name: "Qwen Code".into(),
        category: "开发工具".into(),
        icon: "qwen_code".into(),
        paths: vec![
            "%USERPROFILE%\\.qwen".into(),
            "%LOCALAPPDATA%\\Alibaba\\QwenCode".into(),
            "%APPDATA%\\QwenCode".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["qwen.exe".into(), "qwen-code.exe".into()],
        description: "通义灵码CLI配置与缓存".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("aider".into(), ToolDefinition {
        id: "aider".into(),
        name: "Aider".into(),
        category: "开发工具".into(),
        icon: "aider".into(),
        paths: vec![
            "%USERPROFILE%\\.aider".into(),
            "%LOCALAPPDATA%\\aider".into(),
        ],
        env_vars: vec!["AIDER_CONFIG_DIR".into()],
        strategy: MigrationStrategy::Junction,
        processes: vec!["aider.exe".into()],
        description: "Aider AI编程CLI配置与缓存".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("tabby".into(), ToolDefinition {
        id: "tabby".into(),
        name: "Tabby Terminal".into(),
        category: "开发工具".into(),
        icon: "tabby".into(),
        paths: vec![
            "%APPDATA%\\Tabby".into(),
            "%LOCALAPPDATA%\\Tabby".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["Tabby.exe".into()],
        description: "Tabby终端配置与插件缓存".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("hyper".into(), ToolDefinition {
        id: "hyper".into(),
        name: "Hyper Terminal".into(),
        category: "开发工具".into(),
        icon: "hyper".into(),
        paths: vec![
            "%APPDATA%\\Hyper".into(),
            "%LOCALAPPDATA%\\Hyper".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["Hyper.exe".into()],
        description: "Hyper终端配置与插件缓存".into(),
        risk_level: RiskLevel::Medium,
    });

    // ==================== AI 龙虾/Claw 智能体 ====================

    map.insert("openclaw".into(), ToolDefinition {
        id: "openclaw".into(),
        name: "OpenClaw".into(),
        category: "AI 智能体".into(),
        icon: "claw".into(),
        paths: vec![
            "%USERPROFILE%\\.openclaw".into(),
            "%LOCALAPPDATA%\\OpenClaw".into(),
            "%APPDATA%\\OpenClaw".into(),
            "%USERPROFILE%\\.clawdbot".into(),
            "%USERPROFILE%\\.moltbot".into(),
            "%USERPROFILE%\\.molthub".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["openclaw.exe".into()],
        description: "OpenClaw 本地AI智能体 (配置+会话+记忆+技能包)".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("qclaw".into(), ToolDefinition {
        id: "qclaw".into(),
        name: "QClaw".into(),
        category: "AI 智能体".into(),
        icon: "claw".into(),
        paths: vec![
            "%APPDATA%\\QClaw".into(),
            "%LOCALAPPDATA%\\QClaw".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["QClaw.exe".into()],
        description: "腾讯QClaw AI智能体 (配置+数据+缓存)".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("kimi_claw".into(), ToolDefinition {
        id: "kimi_claw".into(),
        name: "Kimi Claw".into(),
        category: "AI 智能体".into(),
        icon: "claw".into(),
        paths: vec![
            "%USERPROFILE%\\.kimi".into(),
            "%LOCALAPPDATA%\\Kimi".into(),
            "%APPDATA%\\Kimi".into(),
        ],
        env_vars: vec!["KIMI_SHARE_DIR".into()],
        strategy: MigrationStrategy::Junction,
        processes: vec!["kimi.exe".into(), "Kimi.exe".into()],
        description: "Kimi Claw AI智能体 (配置+会话+记忆)".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("workbuddy".into(), ToolDefinition {
        id: "workbuddy".into(),
        name: "WorkBuddy".into(),
        category: "AI 智能体".into(),
        icon: "claw".into(),
        paths: vec![
            "%USERPROFILE%\\.workbuddy".into(),
            "%APPDATA%\\WorkBuddy".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["WorkBuddy.exe".into()],
        description: "WorkBuddy AI工作伙伴 (配置+技能+知识库缓存)".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("qoderworkcn".into(), ToolDefinition {
        id: "qoderworkcn".into(),
        name: "QoderWork CN".into(),
        category: "AI 智能体".into(),
        icon: "claw".into(),
        paths: vec![
            "%USERPROFILE%\\.qoderworkcn".into(),
            "%USERPROFILE%\\.qoderwork".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["QoderWork.exe".into()],
        description: "QoderWork CN AI桌面助手 (数据+缓存+技能)".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("toclaw".into(), ToolDefinition {
        id: "toclaw".into(),
        name: "ToClaw".into(),
        category: "AI 智能体".into(),
        icon: "claw".into(),
        paths: vec![
            "%APPDATA%\\ToDesk\\ToClaw".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["ToClaw.exe".into()],
        description: "ToClaw AI智能体 (配置+缓存)".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("autoclaw".into(), ToolDefinition {
        id: "autoclaw".into(),
        name: "AutoClaw".into(),
        category: "AI 智能体".into(),
        icon: "claw".into(),
        paths: vec![
            "%USERPROFILE%\\.openclaw-autoclaw".into(),
            "%USERPROFILE%\\.autoclaw".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["autoclaw.exe".into(), "AutoClaw.exe".into()],
        description: "智谱AutoClaw AI智能体 (配置+工作区+技能)".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("copaw".into(), ToolDefinition {
        id: "copaw".into(),
        name: "CoPaw".into(),
        category: "AI 智能体".into(),
        icon: "claw".into(),
        paths: vec![
            "%USERPROFILE%\\.copaw".into(),
            "%LOCALAPPDATA%\\CoPaw".into(),
            "%APPDATA%\\CoPaw".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["copaw.exe".into(), "CoPaw.exe".into()],
        description: "CoPaw AI智能体 (配置+数据+缓存)".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("stepclaw".into(), ToolDefinition {
        id: "stepclaw".into(),
        name: "StepClaw".into(),
        category: "AI 智能体".into(),
        icon: "claw".into(),
        paths: vec![
            "%USERPROFILE%\\.stepclaw".into(),
            "%LOCALAPPDATA%\\StepClaw".into(),
            "%APPDATA%\\StepClaw".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["StepClaw.exe".into()],
        description: "阶跃星辰StepClaw桌面AI智能体".into(),
        risk_level: RiskLevel::High,
    });

    // ==================== 其他 IDE / 编辑器 ====================

    map.insert("hbuilderx".into(), ToolDefinition {
        id: "hbuilderx".into(),
        name: "HBuilderX".into(),
        category: "开发工具".into(),
        icon: "hbuilderx".into(),
        paths: vec![
            "%LOCALAPPDATA%\\HBuilder X".into(),
            "%APPDATA%\\HBuilder X".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["HBuilderX.exe".into()],
        description: "HBuilderX IDE配置+插件+缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("sublime_text".into(), ToolDefinition {
        id: "sublime_text".into(),
        name: "Sublime Text".into(),
        category: "开发工具".into(),
        icon: "sublime".into(),
        paths: vec![
            "%APPDATA%\\Sublime Text".into(),
            "%LOCALAPPDATA%\\Sublime Text".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["sublime_text.exe".into()],
        description: "Sublime Text编辑器配置+插件+缓存".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("notepad_plus".into(), ToolDefinition {
        id: "notepad_plus".into(),
        name: "Notepad++".into(),
        category: "开发工具".into(),
        icon: "notepad".into(),
        paths: vec![
            "%APPDATA%\\Notepad++".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["notepad++.exe".into()],
        description: "Notepad++编辑器配置+插件".into(),
        risk_level: RiskLevel::Medium,
    });

    // ==================== 音乐软件 ====================

    map.insert("spotify".into(), ToolDefinition {
        id: "spotify".into(),
        name: "Spotify".into(),
        category: "音视频软件".into(),
        icon: "spotify".into(),
        paths: vec![
            "%APPDATA%\\Spotify".into(),
            "%LOCALAPPDATA%\\Spotify".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["Spotify.exe".into()],
        description: "Spotify音乐缓存+配置".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("netease_music".into(), ToolDefinition {
        id: "netease_music".into(),
        name: "网易云音乐".into(),
        category: "音视频软件".into(),
        icon: "netease".into(),
        paths: vec![
            "%LOCALAPPDATA%\\NeteaseCloudMusic".into(),
            "%APPDATA%\\NeteaseCloudMusic".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["cloudmusic.exe".into()],
        description: "网易云音乐缓存+下载".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("qq_music".into(), ToolDefinition {
        id: "qq_music".into(),
        name: "QQ音乐".into(),
        category: "音视频软件".into(),
        icon: "qqmusic".into(),
        paths: vec![
            "%APPDATA%\\Tencent\\QQMusic".into(),
            "%LOCALAPPDATA%\\Tencent\\QQMusic".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["QQMusic.exe".into()],
        description: "QQ音乐缓存+下载".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("kugou".into(), ToolDefinition {
        id: "kugou".into(),
        name: "酷狗音乐".into(),
        category: "音视频软件".into(),
        icon: "kugou".into(),
        paths: vec![
            "%APPDATA%\\KuGou".into(),
            "%LOCALAPPDATA%\\KuGou".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["KuGou.exe".into()],
        description: "酷狗音乐缓存+下载".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("kuwo".into(), ToolDefinition {
        id: "kuwo".into(),
        name: "酷我音乐".into(),
        category: "音视频软件".into(),
        icon: "kuwo".into(),
        paths: vec![
            "%APPDATA%\\KuWo".into(),
            "%LOCALAPPDATA%\\KuWo".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["kuwo.exe".into()],
        description: "酷我音乐缓存+下载".into(),
        risk_level: RiskLevel::High,
    });

    // ==================== 视频播放器 ====================

    map.insert("potplayer".into(), ToolDefinition {
        id: "potplayer".into(),
        name: "PotPlayer".into(),
        category: "音视频软件".into(),
        icon: "potplayer".into(),
        paths: vec![
            "%APPDATA%\\PotPlayerMini64".into(),
            "%APPDATA%\\Daum\\PotPlayer".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["PotPlayerMini64.exe".into(), "PotPlayerMini.exe".into()],
        description: "PotPlayer播放器配置+缓存".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("vlc".into(), ToolDefinition {
        id: "vlc".into(),
        name: "VLC".into(),
        category: "音视频软件".into(),
        icon: "vlc".into(),
        paths: vec![
            "%APPDATA%\\vlc".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["vlc.exe".into()],
        description: "VLC播放器配置+缓存".into(),
        risk_level: RiskLevel::Medium,
    });

    // ==================== 其他工具 ====================

    map.insert("obs".into(), ToolDefinition {
        id: "obs".into(),
        name: "OBS Studio".into(),
        category: "其它软件".into(),
        icon: "obs".into(),
        paths: vec![
            "%APPDATA%\\obs-studio".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["obs64.exe".into(), "obs32.exe".into()],
        description: "OBS Studio录屏配置+场景".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("windows_terminal".into(), ToolDefinition {
        id: "windows_terminal".into(),
        name: "Windows Terminal".into(),
        category: "开发工具".into(),
        icon: "terminal".into(),
        paths: vec![
            "%LOCALAPPDATA%\\Packages\\Microsoft.WindowsTerminal_8wekyb3d8bbwe".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["WindowsTerminal.exe".into()],
        description: "Windows Terminal配置+缓存".into(),
        risk_level: RiskLevel::Medium,
    });

    // ==================== 视频制作工具 ====================

    map.insert("davinci_resolve".into(), ToolDefinition {
        id: "davinci_resolve".into(),
        name: "DaVinci Resolve".into(),
        category: "音视频软件".into(),
        icon: "davinci".into(),
        paths: vec![
            "%APPDATA%\\Blackmagic Design\\DaVinci Resolve".into(),
            "%PROGRAMDATA%\\Blackmagic Design\\DaVinci Resolve".into(),
            "%LOCALAPPDATA%\\Blackmagic Design\\DaVinci Resolve".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["Resolve.exe".into()],
        description: "DaVinci Resolve达芬奇调色剪辑缓存+配置".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("bijian".into(), ToolDefinition {
        id: "bijian".into(),
        name: "必剪".into(),
        category: "音视频软件".into(),
        icon: "bijian".into(),
        paths: vec![
            "%APPDATA%\\bilibili\\必剪".into(),
            "%LOCALAPPDATA%\\bilibili\\必剪".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["bijian.exe".into()],
        description: "B站必剪视频编辑器缓存+草稿".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("capcut".into(), ToolDefinition {
        id: "capcut".into(),
        name: "CapCut".into(),
        category: "音视频软件".into(),
        icon: "capcut".into(),
        paths: vec![
            "%APPDATA%\\CapCut".into(),
            "%LOCALAPPDATA%\\CapCut".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["CapCut.exe".into()],
        description: "CapCut剪映国际版缓存+草稿".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("handbrake".into(), ToolDefinition {
        id: "handbrake".into(),
        name: "HandBrake".into(),
        category: "音视频软件".into(),
        icon: "handbrake".into(),
        paths: vec![
            "%APPDATA%\\HandBrake".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["HandBrake.exe".into()],
        description: "HandBrake视频转码工具配置+预设".into(),
        risk_level: RiskLevel::Medium,
    });

    // ==================== 图片处理软件 ====================

    map.insert("lightroom".into(), ToolDefinition {
        id: "lightroom".into(),
        name: "Adobe Lightroom".into(),
        category: "图形设计软件".into(),
        icon: "adobe".into(),
        paths: vec![
            "%APPDATA%\\Adobe\\Lightroom".into(),
            "%LOCALAPPDATA%\\Adobe\\Lightroom".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["lightroom.exe".into()],
        description: "Adobe Lightroom照片编辑缓存+预设".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("illustrator".into(), ToolDefinition {
        id: "illustrator".into(),
        name: "Adobe Illustrator".into(),
        category: "图形设计软件".into(),
        icon: "adobe".into(),
        paths: vec![
            "%APPDATA%\\Adobe\\Illustrator".into(),
            "%LOCALAPPDATA%\\Adobe\\Illustrator".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["illustrator.exe".into()],
        description: "Adobe Illustrator矢量设计缓存+配置".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("indesign".into(), ToolDefinition {
        id: "indesign".into(),
        name: "Adobe InDesign".into(),
        category: "图形设计软件".into(),
        icon: "adobe".into(),
        paths: vec![
            "%APPDATA%\\Adobe\\InDesign".into(),
            "%LOCALAPPDATA%\\Adobe\\InDesign".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["InDesign.exe".into()],
        description: "Adobe InDesign排版设计缓存+配置".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("audition".into(), ToolDefinition {
        id: "audition".into(),
        name: "Adobe Audition".into(),
        category: "音视频软件".into(),
        icon: "adobe".into(),
        paths: vec![
            "%APPDATA%\\Adobe\\Audition".into(),
            "%LOCALAPPDATA%\\Adobe\\Audition".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["adobe audition.exe".into()],
        description: "Adobe Audition音频编辑缓存+配置".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("affinity_photo".into(), ToolDefinition {
        id: "affinity_photo".into(),
        name: "Affinity Photo".into(),
        category: "图形设计软件".into(),
        icon: "affinity".into(),
        paths: vec![
            "%APPDATA%\\Affinity\\Photo".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["Photo.exe".into()],
        description: "Affinity Photo照片编辑缓存+配置".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("affinity_designer".into(), ToolDefinition {
        id: "affinity_designer".into(),
        name: "Affinity Designer".into(),
        category: "图形设计软件".into(),
        icon: "affinity".into(),
        paths: vec![
            "%APPDATA%\\Affinity\\Designer".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["Designer.exe".into()],
        description: "Affinity Designer矢量设计缓存+配置".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("coreldraw".into(), ToolDefinition {
        id: "coreldraw".into(),
        name: "CorelDRAW".into(),
        category: "图形设计软件".into(),
        icon: "coreldraw".into(),
        paths: vec![
            "%APPDATA%\\Corel".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["CorelDRW.exe".into()],
        description: "CorelDRAW矢量设计缓存+配置".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("meitu".into(), ToolDefinition {
        id: "meitu".into(),
        name: "美图秀秀".into(),
        category: "图形设计软件".into(),
        icon: "meitu".into(),
        paths: vec![
            "%APPDATA%\\Meitu\\MeituPic".into(),
            "%LOCALAPPDATA%\\Meitu".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["MeituPic.exe".into()],
        description: "美图秀秀图片编辑缓存+配置".into(),
        risk_level: RiskLevel::Medium,
    });

    map.insert("capture_one".into(), ToolDefinition {
        id: "capture_one".into(),
        name: "Capture One".into(),
        category: "图形设计软件".into(),
        icon: "captureone".into(),
        paths: vec![
            "%APPDATA%\\Capture One".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["Capture One.exe".into()],
        description: "Capture One专业RAW编辑缓存+配置".into(),
        risk_level: RiskLevel::High,
    });

    map.insert("luminar".into(), ToolDefinition {
        id: "luminar".into(),
        name: "Luminar".into(),
        category: "图形设计软件".into(),
        icon: "luminar".into(),
        paths: vec![
            "%APPDATA%\\Luminar".into(),
            "%LOCALAPPDATA%\\Luminar".into(),
        ],
        env_vars: vec![],
        strategy: MigrationStrategy::Junction,
        processes: vec!["Luminar.exe".into()],
        description: "Luminar AI照片编辑缓存+配置".into(),
        risk_level: RiskLevel::High,
    });

    map
}
