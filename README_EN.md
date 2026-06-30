# DriveClean / C盘小清新

<p align="center">
  <strong>One-Click Cache Migration Tool for Productivity Software</strong>
</p>

<p align="center">
  Migrate caches from 90+ productivity tools off your C drive — reclaim disk space instantly
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

## Overview

**DriveClean** is a C-drive space optimization tool designed for Windows users. It intelligently scans and migrates cache directories from 90+ productivity tools (dev tools, servers, compilers, AI agents, graphic design, audio/video, office software, and more) to non-C-drive partitions. Using three migration strategies — Junction symlinks, environment variables, and virtual disks — it achieves seamless migration: your software runs as usual, and C-drive space is freed instantly.

## Key Features

| Feature | Description |
|---------|-------------|
| **Smart Scan** | Auto-detect 90+ installed productivity tools and their cache directories, with real-time space usage display |
| **One-Click Migration** | Select target tools and migration strategy, migrate caches to D/E/F drives in one click |
| **Risk Assessment** | Each tool is tagged with Low/Medium/High risk levels; high-risk operations require admin confirmation |
| **Instant Rollback** | All migrations can be rolled back with one click — zero data loss guaranteed |
| **Disk Monitoring** | Real-time C-drive capacity display and remaining space, with disk pressure alerts |
| **Auto Update** | Built-in version detection and auto-update with hash verification for security |

## Supported Tools (90+)

### Development Tools (45+)
- **Package Managers**: NPM, Yarn, PNPM, Pip, Conda, Poetry/Pipenv, NuGet, Conan, Flutter/Dart Pub
- **Version Managers**: nvm-windows, pyenv-win
- **Build Tools**: Maven, Gradle, Rust (rustup + cargo), Go (GOPATH + GOCACHE), MinGW/MSYS2
- **IDEs/Editors**: JetBrains Suite, VS Code, Cursor, Vim/Neovim, Zed
- **AI IDEs**: LM Studio, GitHub Copilot, Kiro, Antigravity IDE, Trae, Qoder
- **Mobile Dev**: Android SDK, Android AVD, Android Studio, Flutter
- **Game Engines**: Unity, Unreal Engine
- **API Tools**: Postman/Apifox, Insomnia
- **Git Clients**: Git LFS, Sourcetree, GitKraken
- **Terminal Tools**: Warp Terminal, Tabby Terminal, Hyper Terminal
- **IaC Tools**: Terraform/Pulumi, LocalStack
- **Local LLMs**: Ollama, Hugging Face

### Servers (12+)
- **Databases**: MySQL/MariaDB, SQL Server, PostgreSQL, Redis, Elasticsearch
- **Message Queues**: RabbitMQ
- **Web Servers**: Nginx, Apache Httpd, IIS
- **Containers/Virtualization**: Docker Desktop (WSL2), WSL2, XAMPP/phpStudy

### AI Agents (9+)
- OpenClaw, QClaw, Kimi Claw, WorkBuddy, QoderWork CN, ToClaw, AutoClaw, CoPaw, StepClaw

### Graphic Design (6+)
- Adobe Photoshop, Premiere Pro, After Effects, Stable Diffusion, Blender, Figma

### Audio/Video (1+)
- JianyingPro (剪映专业版)

### Office & Documents (3+)
- WPS Office, OneDrive, Baidu Netdisk

### Others (11+)
- Chrome/Edge Browser, Claude Desktop, Claude Code CLI, Codex CLI, Aider, QoderWork, VirtualBox/VMware, Vagrant, Hyper-V, Android Emulators, Prometheus/Grafana, Filebeat/Logstash

## Migration Strategies

| Strategy | Mechanism | Use Case |
|----------|-----------|----------|
| **Junction Symlink** | Create NTFS Junction pointing to new path — software accesses seamlessly | Fixed-path cache dirs (IDEs, browsers, etc.) |
| **Environment Variable** | Modify system/user env vars to point to new path | Configurable tools (npm, pip, cargo, etc.) |
| **Virtual Disk (VHDX)** | Create VHDX virtual disk mounted at original path | Large virtualization data (Docker WSL2, VirtualBox, etc.) |

## Technical Architecture

| Layer | Technology | Description |
|-------|------------|-------------|
| **Frontend** | Vue 3 + TypeScript | Tool detection panel, migration UI, C-drive monitor |
| **Backend** | Rust (Tauri 2) | File system ops, process detection, Junction creation, rollback management |
| **Database** | SQLite (rusqlite) | Migration record persistence for rollback queries |
| **Packaging** | NSIS | Windows installer with install/uninstall support |
| **Updates** | reqwest + sha2 | HTTP download + SHA256 hash verification |

### Rust Backend Modules

| Module | Responsibility |
|--------|---------------|
| `tool_map.rs` | 90+ tool definitions: paths, env vars, strategies, risk levels |
| `detect.rs` | Scan locally installed tools and calculate cache space usage |
| `migrate.rs` | Execute Junction / env var / VHDX migration strategies |
| `rollback.rs` | One-click rollback to restore original paths |
| `check.rs` | Post-migration verification ensuring normal operation |
| `admin.rs` | Admin privilege detection and elevation |
| `updater.rs` | Version detection, update download, hash verification |
| `db.rs` | SQLite migration record CRUD |
| `logger.rs` | Logging |
| `path_util.rs` | Windows path utility functions |

## Quick Start

### Download & Install

Visit [Releases](https://github.com/HongJuZi/hjz-c-drive-cleaner/releases) to download the latest Windows installer.

### Build from Source

#### Prerequisites

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://www.rust-lang.org/tools/install) 1.70+
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

#### Build Steps

```bash
# Clone the repository
git clone https://github.com/HongJuZi/hjz-c-drive-cleaner.git
cd hjz-c-drive-cleaner

# Install frontend dependencies
npm install

# Run in development mode
npm run tauri dev

# Production build
npm run tauri build
```

Build output is located at `src-tauri/target/release/bundle/nsis/`.

### Usage Workflow

1. **Scan & Detect** — Launch the app; it auto-scans 90+ tools and their cache usage
2. **Select Migration** — Choose target tools, specify destination drive and strategy
3. **Confirm & Execute** — High-risk operations require admin confirmation; click to migrate
4. **Complete & Reclaim** — Migration done, C-drive space freed, software runs normally

To restore, simply click "Rollback" to revert all migrations.

## Project Structure

```
hjz-c-drive-cleaner/
├── src/                    # Vue 3 frontend
│   ├── App.vue             # Main UI (tool panel, migration ops, C-drive monitor)
│   └── main.ts             # Entry point
├── src-tauri/              # Rust + Tauri backend
│   ├── src/
│   │   ├── main.rs         # Tauri entry + command registration
│   │   ├── tool_map.rs     # 90+ tool definitions
│   │   ├── detect.rs       # Tool detection & scanning
│   │   ├── migrate.rs      # Three migration strategy implementations
│   │   ├── rollback.rs     # Rollback & recovery
│   │   ├── check.rs        # Migration verification
│   │   ├── admin.rs        # Admin privileges
│   │   ├── updater.rs      # Auto-update
│   │   ├── db.rs           # SQLite database
│   │   ├── logger.rs       # Logging
│   │   └── path_util.rs    # Path utilities
│   ├── Cargo.toml          # Rust dependencies
│   └── tauri.conf.json     # Tauri configuration
├── html/                   # Product website
│   └── index.html          # Bilingual (zh/en) website
├── dist/                   # Frontend build output
├── public/                 # Static assets
├── package.json            # Node dependencies
├── vite.config.ts          # Vite configuration
├── tsconfig.json           # TypeScript configuration
├── README.md               # Chinese documentation
├── README_EN.md            # English documentation (this file)
└── LICENSE                 # MIT License
```

## FAQ

**Q: Will software still work after migration?**

A: Yes. Junction symlinks and environment variable changes are completely transparent to the software — it accesses the original path and is automatically redirected. The virtual disk strategy mounts a VHDX at the original path, equivalent to the real location.

**Q: Will my data be lost?**

A: No. Migration simply moves cache directories to a new location and creates a link. All original files are preserved intact. Rollback restores everything with one click.

**Q: What about high-risk operations?**

A: High-risk tools (databases, IDE configs, etc.) require admin confirmation. Risk warnings and estimated space savings are shown before execution.

**Q: Which Windows versions are supported?**

A: Windows 10/11 with NTFS file system (Junction symlinks require NTFS).

## Contributing

Contributions are welcome! Please submit Issues or Pull Requests:

1. Fork this repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Commit your changes: `git commit -m 'Add my feature'`
4. Push to the branch: `git push origin feature/my-feature`
5. Submit a Pull Request

## License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).

```
MIT License

Copyright (c) 2024-2026 Hunan HongJuZi Technology Co., Ltd.

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

## Contact

- **Company**: Hunan HongJuZi Technology Co., Ltd. (湖南红橘子科技有限公司)
- **Email**: xjiujiu@foxmail.com
- **GitHub**: [https://github.com/HongJuZi/hjz-c-drive-cleaner](https://github.com/HongJuZi/hjz-c-drive-cleaner)
- **Website**: [www.hongjuzi.com.cn](https://www.hongjuzi.com.cn)
- **Address**: Huaihua, Hunan, China

---

<p align="center">
  <strong>DriveClean — Reclaim your C drive, effortlessly</strong>
</p>
