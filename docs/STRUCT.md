# TCM-Library-Search 架构设计与分层规范 (STRUCT)

## 1. 总体架构拓扑

TCM-Library-Search 采用 **单仓库多包 (Cargo Workspace Monorepo)** 架构，将数据沉淀、核心检索算力与用户界面解耦，形成分层清晰的单向依赖流：

```text
┌────────────────────────────────────────────────────────┐
│                   Desktop Application                  │
│   (SolidJS + TypeScript + Vite + CSS Modules)          │
│   - ReaderView (宣纸古籍排版 / 正文·古注·白话三层对照)     │
│   - SearchView (11维结构化检索 / 关键词模糊高亮)          │
│   - ImageViewer (药图 / 舌象 / 穴位图集与灯箱预览)        │
└───────────────────────────┬────────────────────────────┘
                            │ Tauri IPC (Commands & Events)
┌───────────────────────────▼────────────────────────────┐
│                    Tauri Host Layer                    │
│   (crates/desktop/src-tauri)                           │
│   - IPC Dispatcher (参数序列化与错误安全映射)            │
│   - Asset Safe Protocol (安全路径校验与图片流读取)        │
│   - Corpus Lifecycle Manager (异步预热与状态管理)       │
└───────────────────────────┬────────────────────────────┘
                            │ Rust Direct Dependency
┌───────────────────────────▼────────────────────────────┐
│                 tcm_library_core (Pure Rust)           │
│   (crates/core)                                        │
│   - Frontmatter & Markdown Parser (无损三层正文切分)    │
│   - Inverted Index Engine (倒排索引与维度评分匹配)      │
│   - Query Matcher & Filter (跨维度组合查询)            │
└───────────────────────────┬────────────────────────────┘
                            │ Reads file system / manifest
┌───────────────────────────▼────────────────────────────┐
│               Corpus Repository (Submodule)            │
│   (corpus/ - https://github.com/moxiaoqin12345/12345)  │
│   - 10 大分类 / 47 个子分类 Markdown 原文              │
│   - manifest.json 机读清单与受控词表                   │
└────────────────────────────────────────────────────────┘
```

---

## 2. 模块职责与边界

### 2.1 `corpus/`（知识库子模块）
- **职责**：只读数据源，由独立仓库维护中医经典医籍（内经、伤寒、金匮、温病、本草）、现代临床指南与方药数据。
- **边界**：不包含编译代码，只包含遵循规范的 Markdown 文件、Frontmatter YAML 元数据与 `manifest.json`。

### 2.2 `crates/core`（纯 Rust 算力与索引库）
- **职责**：
  - **Parser 模块**：解析 Markdown 文件中的 YAML Frontmatter，分离 `【原文】`、`【古注】`、`【白话提要】`。
  - **Index 模块**：读取 `manifest.json` 或遍历条目建立内存倒排索引（Inverted Index），加速多维复合检索。
  - **Matcher 模块**：执行 11 维中医药学维度（证型、治法、病症、症状、方名、药名、腧穴、经络、四气、五味、归经）的打分与权重重排。
- **边界**：
  - 无 IO/OS 特性依赖，只通过入参获取路径或字符串。
  - 所有公开类型和接口均由 `crates/core/src/lib.rs` 顶层统一收敛。

### 2.3 `crates/desktop/src-tauri`（Tauri 宿主与防腐层）
- **职责**：
  - 管理 `CorpusManager` 的全局生命周期与异步后台预热。
  - 注册 Tauri IPC Commands（如 `search_entries`, `get_entry_detail`, `list_categories`）。
  - 提供安全的媒体资源协议（`tcm-asset://` 或 Tauri Asset Protocol），防止路径遍历漏洞。

### 2.4 `crates/desktop/src`（前端 SolidJS 交互视图）
- **职责**：
  - **ReaderView**：沉浸式阅读器，支持日间宣纸浅色、暗夜竹简深色、字体字号调节、三层内容折叠/并排展示。
  - **SearchView**：快速多维检索工作台，提供分类标签筛选、临床条件多选胶囊（Pill Filters）与搜索结果高亮。
  - **ImageViewer**：本草标本图、经穴插图自适应展示，点击激活全屏灯箱（Lightbox），支持放大、平移与旋转。

---

## 3. 数据流设计

1. **应用启动**：
   - 宿主读取 `corpus/manifest.json`（若未生成则异步扫描 `corpus/library/` 自动生成）。
   - Rust Core 初始化内存索引，耗时控制在 50ms 内完成热载。
2. **用户检索**：
   - 前端触发 `search_entries(query_params)`。
   - Tauri IPC 调用 Core Matcher，进行维度过滤与加权匹配，返回轻量 `SearchResultItem` 列表。
3. **条目精读**：
   - 用户选中条目，前端调用 `get_entry_detail(id)`。
   - Core 检索条目完整三层文本并解析其中的图片资源路径，返回 `TcmEntryDetail`。
   - 前端动态渲染古籍排版，并将图片路径转换为安全 Asset URL 呈现。
