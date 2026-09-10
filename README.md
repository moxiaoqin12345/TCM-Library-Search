# TCM-Library-Search · 中医典籍知识智能文库与阅读器

<div align="center">

**面向中医古籍与现代临床知识的高性能结构化检索库与沉浸式桌面阅读应用**

[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.80+-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-v2-24C8DB.svg)](https://v2.tauri.app/)
[![SolidJS](https://img.shields.io/badge/SolidJS-1.8+-446b9e.svg)](https://www.solidjs.com/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.0+-3178C6.svg)](https://www.typescriptlang.org/)

</div>

---

## 📖 项目简介

**TCM-Library-Search** 是一套专为中医从业者、中医药学者、古籍研习者及传统医学爱好者打造的高性能结构化典籍检索与沉浸式阅读系统。

项目采用 **Cargo Workspace Monorepo** 分层架构：
- **纯 Rust 检索与索引算力内核 (`crates/core`)**：负责典籍 YAML Frontmatter 结构化元数据解析、倒排索引构建、11 维中医临床维度（证型、治法、病症、症状、方名、药名、腧穴、经络、四气、五味、归经）复合匹配、毫秒级全文模糊检索与结果加权评分。
- **现代化跨平台桌面应用 (`crates/desktop`)**：采用 Tauri v2 + SolidJS + TypeScript 构建，兼具原生级别的内存/算力效率与现代 Web 交互体验。提供宣纸/竹简等多套典籍雅致主题、正文/古注/白话三层对照排版、插图原图灯箱缩放以及本地离线即查即用的检索工作台。
- **动态古籍文库子模块 (`corpus/`)**：作为 Git Submodule 挂载独立维护的中医知识库（涵盖 10 大分类、47 个专业子类），支持持续演进更新与零侵入同步。

---

## 🌟 核心特性

1. **三层互文精读排版**：
   - **原文层**：保持经典古籍原本风貌，字句严谨校订。
   - **古注层**：汇集历代名家先贤注释（如《本草纲目》《伤寒论注》等），随文研读。
   - **白话提要层**：现代通俗精要总结与方药药理精释，语义清晰易懂。

2. **11 维中医结构化检索**：
   - 支持跨维度组合检索：`证型` (如 太阳中风)、`治法` (如 解表散寒)、`病症`、`症状`、`方名`、`药名`、`腧穴`、`经络`、`四气`、`五味`、`归经` 等。
   - 跨字段 AND、同字段 OR，结合特异性加权算法，精准定位辨证论治典籍出处。

3. **图文并茂的插图阅览体系**：
   - 深度兼容药材标本图、经络穴位走向图、舌苔脉象图解与古籍书影。
   - 原生支持 Markdown 相对路径寻址、Asset 自定义安全加载、高清图集灯箱（Lightbox）缩放预览与无图优雅降级。

4. **沉浸式阅读与个性化视觉**：
   - 提供**素白纸墨**、**仿古宣纸**、**暮墨玄竹**等多种护眼阅读主题。
   - 支持楷体/宋体字号、行距自定义调节，带来纸质古籍般的愉悦阅读享受。

5. **纯本地离线与极速响应**：
   - 所有文本与索引常驻本地，毫秒级即时检索，无需外部联网依赖，守护医疗知识查询的私密与可靠。

---

## 📂 仓库目录结构

```text
TCM-Library-Search/
├── .github/                 # 自动化 CI / CD 工作流 (代码风格、Clippy静态分析、自动化测试)
├── docs/                    # 架构与工程规范
│   ├── STRUCT.md            # 项目架构设计与分层规范
│   ├── IMAGE_SPEC.md        # 典籍插图与多媒体资源接入规范
│   ├── API.md               # Rust Core 与 Tauri IPC 接口契约
│   └── TODO.md              # 迭代路线图与开发任务清单
├── corpus/                  # [Git Submodule] 中医典籍结构化文库 (持续同步更新)
│   ├── manifest.json        # 机读总清单
│   └── library/             # 10 大类古籍与现代中医条目 Markdown
├── crates/
│   ├── core/                # tcm_library_core 纯 Rust 检索与解析内核
│   │   ├── Cargo.toml
│   │   └── src/             # 索引引擎、维度匹配器、Frontmatter解析器
│   └── desktop/             # Tauri v2 + SolidJS 桌面客户端
│       ├── package.json
│       ├── src/             # 前端 SolidJS 视图 (ReaderView, SearchView, ImageViewer)
│       └── src-tauri/       # Tauri v2 宿主后端 (IPC Command、Asset Handler)
├── Cargo.toml               # Workspace 根配置
├── CONTRIBUTING.md          # 协作者贡献准则与开发流程
├── AGENTS.md                # AI Agent 协作强制规范与工程红线
├── .gitignore
└── README.md
```

---

## 🛠️ 快速开始

### 前置环境要求
- [Rust](https://www.rust-lang.org/) (推荐 1.80 或更高版本)
- [Node.js](https://nodejs.org/) (>= 20 LTS)
- [pnpm](https://pnpm.io/) (>= 9)
- [Git](https://git-scm.com/) (支持 Git Submodule)

### 1. 克隆仓库与子模块初始化
```bash
git clone --recurse-submodules https://github.com/moxiaoqin12345/TCM-Library-Search.git
cd TCM-Library-Search

# 若已克隆但未初始化子模块，执行：
git submodule update --init --recursive
```

### 2. 检验与测试 Rust 核心库
```bash
# 检查整个 Rust 工作区
cargo check --workspace

# 运行内核单元测试
cargo test -p tcm_library_core

# 静态分析与格式检查
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

### 3. 启动桌面端开发预览
```bash
cd crates/desktop

# 安装前端依赖
pnpm install

# 启动桌面开发调试模式 (自动启动 Vite 前端服务与 Tauri 原生窗口)
pnpm tauri dev
```

### 4. 生产环境构建与打包
```bash
cd crates/desktop

# 生成各平台安装包 (Windows .msi / .exe, macOS .dmg / .app, Linux .deb / .AppImage)
pnpm tauri build
```

---

## 📚 规范与开发文档

在提交代码前，请完整查阅以下规范文档：
- **[CONTRIBUTING.md](CONTRIBUTING.md)**: 协作者行为准则、分支流程与提交规范。
- **[AGENTS.md](AGENTS.md)**: AI Agent 自动化开发约束与红线守则。
- **[docs/STRUCT.md](docs/STRUCT.md)**: 模块分层、数据流向与边界划分。
- **[docs/IMAGE_SPEC.md](docs/IMAGE_SPEC.md)**: 典籍图谱、舌象插图与媒体资源接入标准。
- **[docs/API.md](docs/API.md)**: Rust 内核与前端 IPC 协议规范。
- **[docs/TODO.md](docs/TODO.md)**: 阶段性开发计划与功能进展。

---

## 📄 开源许可与免责声明

本项目遵循 [MIT License](LICENSE) 开源。

> **免责声明**：本项目收录的中医典籍文献、方剂与临床参考内容仅供学术研讨、古籍学习与科学研究使用，不构成任何形式的直接临床诊断或处方建议。临床用药与针灸治疗请务必遵从执业中医师专业指导。
