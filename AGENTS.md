# TCM-Library-Search Agent 开发强制规则与工程准则

> **重要声明 (MANDATORY)**：
> 所有介入本项目（TCM-Library-Search）的 AI Agent、Copilot 及自动化代码生成助手，在启动任何开发任务前，**必须完整阅读并严格遵守本文件中的所有硬性红线**。任何违反本规范的修改将视为不合规操作并被回滚。

---

## 1. 架构隔离与技术红线（最高优先级）

### 1.1 内核计算与索引库 (`crates/core`) 纯净性红线
- **定位**：`crates/core`（Crate 名称：`tcm_library_core`）是纯中医典籍数据模型、Frontmatter 解析引擎与结构化检索算力内核。
- **零平台/UI依赖**：**绝对禁止**引入任何前端、Web、Tauri、Wasm-bindgen、OS 系统原生窗口、GUI 相关的 crate 或代码。
- **无副作用与跨平台**：内核必须支持纯标准环境（未来保持具备 `no_std` 或跨平台嵌入式编译扩展性），仅负责纯粹的数据结构推导与算法输入输出。
- **API 收敛原则**：所有对外暴露的数据结构、枚举、Trait 及计算检索接口，**必须且只能通过 `crates/core/src/lib.rs` 统一向外导出**。内部子模块（如 `parser`, `index`, `retriever`, `filter` 等）仅暴露给 crate 内部或通过 `pub use` 由顶层收敛导出，严禁外部直接以深层路径引用未收敛的内部私有状态。

### 1.2 客户端依赖与调用方向
- `crates/desktop` 只能通过 Cargo 的本地相对路径依赖 `tcm_library_core`。
- **严禁逆向依赖**：`core` 绝不能感知 `desktop`、Tauri 或任何展示层的存在。
- **IPC 契约防腐**：Tauri Command 暴露的方法必须是轻量防腐层（Facade），负责入参校验、调用 core 内核方法并将结构转换为前端友好的 DTO。

---

## 2. 典籍数据契约与图片接入规范

### 2.1 典籍三层正文不可篡改性
- 古籍正文必须严格保持三层互文结构：
  1. `【原文】`：保持古籍原貌，绝不能被任何自动化过程擅自修改、删减或进行 AI 润色。
  2. `【古注】` / `【阐微】`：历代注家批语，保持客观引证。
  3. `【白话提要】`：通俗白话解读与现代药理/方解归纳。
- 解析器遇到缺失某一层的条目时，必须优雅返回 `None` 或安全默认值，严禁中断解析或抛出运行时异常。

### 2.2 图片与多媒体资源接入原则
- 典籍中的图谱（本草插图、经络穴位走向、舌象脉象图谱、古籍书影）接入必须遵循 [docs/IMAGE_SPEC.md](docs/IMAGE_SPEC.md)。
- **相对路径安全性**：针对 Markdown 中的相对图片引用（如 `![](images/dahuang_01.png)`），必须在 Tauri 宿主层通过严谨的路径沙箱校验（限制在 `corpus/` 或预设静态资源根目录内），严禁发生路径穿越漏洞。
- **降级与容错**：前端图片加载组件必须内置加载骨架屏（Skeleton）、错误占位图（Placeholder）与高清灯箱放大（Lightbox）支持。

---

## 3. 错误处理与类型安全

- `tcm_library_core` 必须通过 `thiserror` 定义统一、语义明确的错误枚举 `CoreError`，并通过统一别名暴露：
  ```rust
  pub type Result<T> = std::result::Result<T, CoreError>;
  ```
- **严禁在生产路径中使用 `unwrap()`、`expect()`、`panic!()`**。所有 IO 异常、YAML 语法错误、越界检索与无效维度参数必须作为 `Result::Err` 返回。
- 所有在前端与 Rust 内核间传递的数据结构，必须完整实现 `serde::Serialize` 与 `serde::Deserialize`，并在前端配对定义强类型 TypeScript `interface`。

---

## 4. 国际化与受控词表 (i18n & Controlled Vocabulary)

- **中医术语与拼音标识**：
  - 代码内部（枚举、字段、Key）采用英文或规范全拼（如 `Zhengxing`, `Zhifa`, `Bingzheng`, `Yaoming`, `Fangming`, `Shuxue`, `Jingluo`）。
  - 用户展示文案、提示词及多语言支持必须统一由前端 i18n 资源文件（`locales/zh-CN.json`、`locales/zh-TW.json`、`locales/en-US.json`）统一管理，严禁在业务计算逻辑中硬编码展示文案作为条件判断依据。
- **受控词表对齐**：检索过滤维度必须与 `corpus/schema` 中的受控词表（Controlled Vocabulary）保持强一致。

---

## 5. 自主品牌与知识产权约束

- **自主品牌定位**：本项目为独立的开源中医药智能典籍检索与阅读平台。在任何注释、文档、Commit 信息及生成代码中，**严禁提及任何内部参考原型项目名称**，保持项目自主性。
- **开源合规**：所有新增引入的第三方依赖必须具有与 MIT 协议兼容的开源许可证。

---

## 6. 提交流程与自动化验收门禁

Agent 在完成任何代码编写后，必须在本地终端执行并通过以下验证流程方可交付：

```bash
# 1. Rust 格式化与静态检查
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings

# 2. 单元测试全量通过
cargo test --workspace

# 3. 前端类型检查与编译构建
cd crates/desktop && pnpm build
```
