# TCM-Library-Search 迭代路线图与开发任务清单 (TODO)

## 📌 阶段规划总览

```text
Phase 1: 工程初始化与规范确立 (当前阶段)
   └── Submodule 挂载、工程目录、开发准则、图片规范确立
Phase 2: Rust 纯内核算力与索引引擎
   └── YAML Frontmatter 解析器、三层切分器、11维中医药倒排索引
Phase 3: Tauri 宿主防腐层与安全资产服务
   └── IPC Facade 接入、相对路径图片沙箱转换、CorpusManager 异步预热
Phase 4: 桌面端现代阅读器与交互工作台
   └── SolidJS 视图、宣纸/竹简双主题排版、11维胶囊检索、图谱灯箱
Phase 5: 混合搜索优化与跨平台发布
   └── 本地向量/RAG 预留、多端打包自动化 (Windows / macOS / Linux)
```

---

## 🎯 详细任务清单

### Phase 1: 工程初始化与规范确立 (Completed)
- [x] 配置根目录 `.gitignore` 与 Git 忽略策略
- [x] 挂载 `https://github.com/moxiaoqin12345/12345.git` 为 `corpus/` Git Submodule
- [x] 编撰完整的面向用户产品文档 `README.md`
- [x] 编撰协作者行为与代码规范文档 `CONTRIBUTING.md`
- [x] 制定 AI Agent 强制工程红线与架构隔离规则 `AGENTS.md`
- [x] 输出技术架构分层文档 `docs/STRUCT.md`
- [x] 制定多媒体插图安全加载与交互规范 `docs/IMAGE_SPEC.md`
- [x] 确立核心 API 契约与 IPC 通信协议 `docs/API.md`

### Phase 2: Rust 核心算力与索引库 (`crates/core`) (Completed)
- [x] 创建 `Cargo.toml` Workspace 结构与 `crates/core` 包定义
- [x] 实现 `parser` 模块：无损提取 Frontmatter 元数据与正文三层标记（原文/古注/白话）
- [x] 实现 `markdown` 图片标签解析器（提取正文行内图片及其图注）
- [x] 实现 `index` 模块：高效解析 `corpus/manifest.json` 与全字段内存倒排索引
- [x] 实现 `matcher` 模块：11 维中医药临床复合条件过滤与特异性加权打分
- [x] 编写核心单元测试（针对药性、方剂、经典条目的匹配与容错测试）

### Phase 3: Tauri 宿主防腐层 (`crates/desktop/src-tauri`) (Completed)
- [x] 配置 Tauri v2 宿主环境与 Cargo 依赖
- [x] 实现 `CorpusManager`：自动定位文库目录（开发环境相对路径与生产打包 Resource 路径）
- [x] 实现安全图片 Asset Protocol / Handler：校验沙箱范围，防止 `../` 路径穿越
- [x] 注册并实现 `search_entries`, `get_entry_detail`, `list_categories` 等 IPC 指令

### Phase 4: 桌面端现代阅读器与交互工作台 (`crates/desktop/src`) (Completed)
- [x] 配置 Vite + SolidJS + TypeScript + CSS Modules 前端工程骨架
- [x] 实现三栏式沉浸交互布局（Nav Rail + 中间多功能面板 + 右侧精读区）
- [x] 下拉式多级筛选器取代滚动胶囊（分类联动、11维证治维度筛选、多模式排序）
- [x] 典籍按书查阅（图书馆面板与书目卡片）
- [x] 正文三层互文开关（独立切换【原文】/【古注】/【白话提要】）
- [x] 字号与行距下拉调节（支持小到超大 5 档及紧凑/舒适/宽松）
- [x] 宣纸仿古浅色与竹简深色护眼双主题切换系统
- [x] 插图安全渲染、骨架屏容错与全屏灯箱模态层（Lightbox 支持滚轮缩放与拖拽）

### Phase 5: 后续演进与发布运维
- [x] 书籍篇章目录树（Chapter Tree）：图书库选书后在中间面板展开该书的卷次/章节树，进行树状逐章点读
- [x] 本地书签与高亮批注持久化（LocalStorage 存储用户标注、随记心得与阅读历史）
- [x] 自动化 CI 工作流（GitHub Actions：cargo fmt, clippy, test 与 pnpm build）
- [x] 初始代码提交与 Git 仓库规范化小粒度 Commit 归档
- [x] 典籍正文全文高亮检索（在原文中高亮当前搜索的关键词与临床症状定位）
- [x] 正文双击/划词词典释义弹窗（中医药专业术语速查与跨书目关联推导）
- [ ] 本地开发运行与打包验证（Tauri Dev 运行调试、本地打包生成安装包）
- [ ] 导出与备份（支持将研读心得、书签与古籍条目导出为 Markdown 或 JSON）
- [ ] 跨平台发布工作流（GitHub Actions Release 自动构建 Windows `.msi`/`.exe`、macOS `.dmg`）

