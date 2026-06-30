# C盘小清新 / DriveClean

<p align="center">
  <strong>生产力软件缓存一键迁移工具</strong>
</p>

<p align="center">
  让 90+ 款生产力软件的缓存一键搬家，告别 C 盘空间不足的烦恼
</p>

<p align="center">
  <a href="https://github.com/HongJuZi/hjz-c-drive-cleaner/releases">
    <img src="https://img.shields.io/github/v/release/HongJuZi/hjz-c-drive-cleaner?style=flat-square&label=Release" alt="Release">
  </a>
  <img src="https://img.shields.io/badge/Platform-Windows-blue?style=flat-square" alt="Platform">
  <img src="https://img.shields.io/badge/Rust-2021-orange?style=flat-square" alt="Rust">
  <img src="https://img.shields.io/badge/Tauri-2.0-green?style=flat-square" alt="Tauri">
  <img src="https://img.shields.io/badge/Vue-3-4FC08D?style=flat-square" alt="Vue">
  <img src="https://img.shields.io/badge/License-MIT-yellow?style=flat-square" alt="License">
</p>

---

## 简介

**C盘小清新**是一款专为 Windows 用户打造的 C 盘空间优化工具。它能智能扫描并一键迁移 90+ 款生产力软件（开发工具、服务器、编译器、AI 智能体、图形设计、音视频、文档办公等）的缓存目录到非 C 盘分区，通过 Junction 软链接、环境变量、虚拟磁盘三种迁移策略，实现无感迁移——软件照常运行，C 盘空间瞬间释放。

## 核心功能

| 功能 | 说明 |
|------|------|
| **智能扫描** | 自动检测已安装的 90+ 款生产力软件及其缓存目录，实时显示占用空间 |
| **一键迁移** | 选择目标工具和迁移策略，一键将缓存迁移到 D/E/F 等非 C 盘分区 |
| **风险评估** | 每个工具标注低/中/高风险等级，高风险操作需管理员权限确认 |
| **一键回滚** | 所有迁移操作均可一键回滚恢复，确保数据安全零损失 |
| **空间监控** | 实时显示 C 盘容量和剩余空间，预警磁盘压力 |
| **自动更新** | 内置版本检测和自动更新机制，支持哈希校验确保安全 |

## 支持的工具（90+ 款）

### 开发工具（45+）
- **包管理器**：NPM、Yarn、PNPM、Pip、Conda、Poetry/Pipenv、NuGet、Conan、Flutter/Dart Pub
- **版本管理**：nvm-windows、pyenv-win
- **构建工具**：Maven、Gradle、Rust (rustup + cargo)、Go (GOPATH + GOCACHE)、MinGW/MSYS2
- **IDE/编辑器**：JetBrains 全家桶、VS Code、Cursor、Vim/Neovim、Zed
- **AI IDE**：LM Studio、GitHub Copilot、Kiro、Antigravity IDE、Trae、Qoder
- **移动开发**：Android SDK、Android AVD、Android Studio、Flutter
- **游戏引擎**：Unity、Unreal Engine
- **API 工具**：Postman/Apifox、Insomnia
- **Git 客户端**：Git LFS、Sourcetree、GitKraken
- **终端工具**：Warp Terminal、Tabby Terminal、Hyper Terminal
- **IaC 工具**：Terraform/Pulumi、LocalStack
- **本地大模型**：Ollama、Hugging Face

### 服务器（12+）
- **数据库**：MySQL/MariaDB、SQL Server、PostgreSQL、Redis、Elasticsearch
- **消息队列**：RabbitMQ
- **Web 服务器**：Nginx、Apache Httpd、IIS
- **容器/虚拟化**：Docker Desktop (WSL2)、WSL2、XAMPP/phpStudy

### AI 智能体（9+）
- OpenClaw、QClaw、Kimi Claw、WorkBuddy、QoderWork CN、ToClaw、AutoClaw、CoPaw、StepClaw

### 图形设计（6+）
- Adobe Photoshop、Premiere Pro、After Effects、Stable Diffusion、Blender、Figma

### 音视频（1+）
- 剪映专业版

### 文档办公（3+）
- WPS Office、OneDrive、百度网盘

### 其它（11+）
- Chrome/Edge 浏览器、Claude 桌面端、Claude Code CLI、Codex CLI、Aider、QoderWork、VirtualBox/VMware、Vagrant、Hyper-V、安卓模拟器、Prometheus/Grafana、Filebeat/Logstash

## 迁移策略

| 策略 | 原理 | 适用场景 |
|------|------|---------|
| **Junction 软链接** | 创建 NTFS Junction 指向新路径，软件无感访问 | 固定路径的缓存目录（IDE、浏览器等） |
| **环境变量** | 修改系统/用户环境变量指向新路径 | 支持配置路径的工具（npm、pip、cargo 等） |
| **虚拟磁盘** | 创建 VHDX 虚拟磁盘挂载到原路径 | 大体积虚拟化数据（Docker WSL2、VirtualBox 等） |

## 技术架构

| 层 | 技术 | 说明 |
|----|------|------|
| **前端** | Vue 3 + TypeScript | 工具检测面板、迁移操作界面、C 盘监控 |
| **后端** | Rust (Tauri 2) | 文件系统操作、进程检测、Junction 创建、回滚管理 |
| **数据库** | SQLite (rusqlite) | 迁移记录持久化，支持回滚查询 |
| **打包** | NSIS | Windows 安装程序，支持安装/卸载 |
| **更新** | reqwest + sha2 | HTTP 下载更新包 + SHA256 校验完整性 |

### Rust 后端模块

| 模块 | 职责 |
|------|------|
| `tool_map.rs` | 90+ 工具定义：路径、环境变量、策略、风险等级 |
| `detect.rs` | 扫描本地已安装工具，计算缓存占用空间 |
| `migrate.rs` | 执行 Junction / 环境变量 / VHDX 三种迁移策略 |
| `rollback.rs` | 一键回滚所有迁移操作，恢复原始路径 |
| `check.rs` | 迁移后验证，确保软件正常运行 |
| `admin.rs` | 管理员权限检测与申请 |
| `updater.rs` | 版本检测、下载更新包、哈希校验 |
| `db.rs` | SQLite 迁移记录 CRUD |
| `logger.rs` | 日志记录 |
| `path_util.rs` | Windows 路径工具函数 |

## 快速开始

### 下载安装

前往 [Releases](https://github.com/HongJuZi/hjz-c-drive-cleaner/releases) 下载最新版本的 Windows 安装包。

### 从源码构建

#### 前置条件

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://www.rust-lang.org/tools/install) 1.70+
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

#### 构建步骤

```bash
# 克隆仓库
git clone https://github.com/HongJuZi/hjz-c-drive-cleaner.git
cd hjz-c-drive-cleaner

# 安装前端依赖
npm install

# 开发模式运行
npm run tauri dev

# 生产构建
npm run tauri build
```

构建产物位于 `src-tauri/target/release/bundle/nsis/` 目录。

### 使用流程

1. **扫描检测** — 启动后自动扫描已安装的 90+ 款工具及缓存占用
2. **选择迁移** — 选择目标工具，指定迁移目标盘符和策略
3. **确认执行** — 高风险操作需管理员权限确认，一键迁移
4. **完成释放** — 迁移完成，C 盘空间释放，软件照常运行

如需恢复，点击"一键回滚"即可还原所有迁移。

## 项目结构

```
hjz-c-drive-cleaner/
├── src/                    # Vue 3 前端
│   ├── App.vue             # 主界面（工具面板、迁移操作、C盘监控）
│   └── main.ts             # 入口
├── src-tauri/              # Rust + Tauri 后端
│   ├── src/
│   │   ├── main.rs         # Tauri 入口 + 命令注册
│   │   ├── tool_map.rs     # 90+ 工具定义
│   │   ├── detect.rs       # 工具检测扫描
│   │   ├── migrate.rs      # 三种迁移策略实现
│   │   ├── rollback.rs     # 回滚恢复
│   │   ├── check.rs        # 迁移验证
│   │   ├── admin.rs        # 管理员权限
│   │   ├── updater.rs      # 自动更新
│   │   ├── db.rs           # SQLite 数据库
│   │   ├── logger.rs       # 日志
│   │   └── path_util.rs    # 路径工具
│   ├── Cargo.toml          # Rust 依赖
│   └── tauri.conf.json     # Tauri 配置
├── html/                   # 产品官网
│   └── index.html          # 中英双语官网
├── dist/                   # 前端构建产物
├── public/                 # 静态资源
├── package.json            # Node 依赖
├── vite.config.ts          # Vite 配置
├── tsconfig.json           # TypeScript 配置
└── README.md               # 本文件
```

## 常见问题

**Q: 迁移后软件还能正常运行吗？**

A: 可以。Junction 软链接和环境变量迁移对软件完全透明，软件访问原路径时自动跳转到新位置。虚拟磁盘策略将 VHDX 挂载到原路径，效果等同。

**Q: 数据会丢失吗？**

A: 不会。迁移操作只是将缓存目录移动到新位置并建立链接，所有原始文件完整保留。回滚操作可一键还原。

**Q: 高风险操作怎么办？**

A: 高风险工具（如数据库、IDE 配置）需要管理员权限确认。操作前会显示风险提示和预计释放空间，确认后才执行。

**Q: 支持哪些 Windows 版本？**

A: Windows 10/11，需 NTFS 文件系统（Junction 软链接依赖 NTFS）。

## 贡献

欢迎贡献！请提交 Issue 或 Pull Request：

1. Fork 本仓库
2. 创建功能分支：`git checkout -b feature/my-feature`
3. 提交修改：`git commit -m 'Add my feature'`
4. 推送分支：`git push origin feature/my-feature`
5. 提交 Pull Request

## 许可证

本项目基于 [MIT 许可证](https://opensource.org/licenses/MIT) 开源。

```
MIT License

Copyright (c) 2024-2026 湖南红橘子科技有限公司

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## 联系我们

- **公司**：湖南红橘子科技有限公司
- **邮箱**：xjiujiu@foxmail.com
- **GitHub**：[https://github.com/HongJuZi/hjz-c-drive-cleaner](https://github.com/HongJuZi/hjz-c-drive-cleaner)
- **官网**：[www.hongjuzi.com.cn](https://www.hongjuzi.com.cn)
- **地址**：湖南怀化

---

<p align="center">
  <strong>C盘小清新 — 让 C 盘回归清爽</strong>
</p>
