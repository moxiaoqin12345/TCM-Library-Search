# TCM-Library-Search 架构设计与分层规范 (STRUCT)

## 1. 总体架构拓扑

TCM-Library-Search 采用 **单仓库多包 (Cargo Workspace Monorepo)** 架构，将数据沉淀、核心检索算力与用户界面解耦，形成分层清晰的单向依赖流：

```text
┌────────────────────────────────────────────────────────┐
│                   Desktop Application                  │
│   (SolidJS + TypeScript + Vite + CSS Modules)          │
│   - ReaderView (宣纸古籍排版 / 正文·古注·白话三层对照)     │
│   - SearchView (11维结构化检索 / 关键词模糊高亮)          │
│   - CompatibilityRadar (M1: 配伍禁忌实时警戒雷达)        │
│   - DiffModal (M2: 分屏/紧凑异文校勘与药量比对)          │
│   - MeridianPanel (M3: 经络循行流注图谱与临床对症推穴)    │
│   - ImageViewer (药图 / 舌象 / 穴位图集与灯箱预览)        │
└───────────────────────────┬────────────────────────────┘
                            │ Tauri IPC (Commands & Events)
┌───────────────────────────▼────────────────────────────┐
│                    Tauri Host Layer                    │
│   (crates/desktop/src-tauri)                           │
│   - IPC Dispatcher (检索/精读/禁忌雷达/异文校勘/经穴拓扑)  │
│   - Asset Safe Protocol (安全路径校验与图片流读取)        │
│   - Corpus Lifecycle Manager (异步预热与状态管理)       │
└───────────────────────────┬────────────────────────────┘
                            │ Rust Direct Dependency
┌───────────────────────────▼────────────────────────────┐
│                 tcm_library_core (Pure Rust)           │
│   (crates/core)                                        │
│   - Compatibility Engine (M1: 十八反/十九畏/妊娠禁忌)     │
│   - Text Diff Engine (M2: LCS 古籍异文与药量校雠算法)     │
│   - Meridian Engine (M3: 十二经脉流注与对症推穴拓扑)     │
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
  - **Compatibility 模块 (M1)**：结构化十八反、十九畏、妊娠禁用/慎用规则库，秒级碰撞判定方药两两配伍冲突。
  - **Diff 模块 (M2)**：基于最长公共子序列 (LCS) 算法，执行古文多版本传抄衍脱、字句互异与药量化裁的精准对比。
  - **Meridian 模块 (M3)**：十二经脉循行流注次序、五输穴/原/络/郄/俞/募要穴数据库，以及基于主治症状的临床智能配穴推导。
- **边界**：
  - **纯纯净计算内核**：绝无 GUI、Tauri、Web 或 OS 平台特定依赖。
  - 所有公开类型和接口均由 `crates/core/src/lib.rs` 顶层统一收敛导出。

### 2.3 `crates/desktop/src-tauri`（Tauri 宿主与防腐层）
- **职责**：
  - 管理 `CorpusManager` 的全局生命周期与异步后台预热。
  - 注册 Tauri IPC Commands（`search_entries`, `get_entry_detail`, `check_compatibility`, `diff_text_versions`, `list_meridians`, `recommend_acupoints` 等）。
  - 提供安全的媒体资源协议与 Data URI 转换，防止路径遍历漏洞。

### 2.4 `crates/desktop/src`（前端 SolidJS 交互视图）
- **职责**：
  - **ReaderView**：沉浸式阅读器，支持宣纸/暮竹双主题、字体字号调节、三层内容折叠与高亮批注。
  - **SearchView**：快速多维检索工作台，提供分类标签筛选、临床条件多选胶囊与搜索结果高亮。
  - **CompatibilityRadar**：实时配伍安全气泡与雷达预警，解析方药中存在的十八反/十九畏与妊娠风险。
  - **DiffModal**：分屏对比 (Split) 与合并比对 (Unified) 双模式，清晰比对不同古籍传抄版本的药味与文句出入。
  - **MeridianPanel**：十二经脉气血流注时间、五行表里、特定穴位多重筛选与临床对症智能推荐。
  - **ImageViewer**：本草标本图、经穴插图自适应展示，点击激活全屏灯箱（Lightbox），支持放大与平移。

---

## 3. 数据流设计

1. **应用启动与热载**：
   - 宿主读取 `corpus/manifest.json`。
   - Rust Core 初始化内存索引，耗时控制在 50ms 内完成热载。
2. **多维检索流**：
   - 前端触发 `search_entries(query)`。
   - Tauri IPC 调用 Core Matcher，进行维度过滤与加权匹配，返回轻量 `SearchResultItem` 列表。
3. **条目精读与实时禁忌雷达 (M1)**：
   - 用户选中条目，前端调用 `get_entry_detail(id)` 渲染三层古籍正文。
   - 前端自动提取该条目方药组成，调用 `check_compatibility(herbs)`，Core 碰撞规则库后向前端推送警示气泡。
4. **版本分屏校雠流 (M2)**：
   - 用户调起异文互校窗口，传入基础本与对照本文本，调用 `diff_text_versions(a, b)`。
   - Core LCS 引擎输出差分 Chunks 并统计异文率，前端即时分屏高亮增删改。
5. **经络腧穴流注与对症推导流 (M3)**：
   - 用户打开经络图谱，IPC 调取 `list_meridians()` 呈现流注时辰与循行定位。
   - 用户输入临床症候（如“牙痛”），调用 `recommend_acupoints("牙痛")`，Core 推荐合谷、内庭等特定要穴。
