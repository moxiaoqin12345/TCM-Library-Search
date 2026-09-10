# TCM-Library-Search 协作者与贡献指南 (CONTRIBUTING)

感谢您关注并参与 **TCM-Library-Search**（中医典籍知识智能文库与阅读器）的建设！无论您是提出建议、撰写文档、优化排版，还是贡献代码、完善检索算法与插图系统，您的每一份贡献都至关重要。

---

## 1. 行为准则 (Code of Conduct)

我们致力于打造一个开放、包容、严谨且学术氛围浓厚的协作环境。请所有参与者：
- 尊重多样性，保持建设性、友善与专业的交流态度。
- 在涉及中医药传统理论与文献考据时，尊重古籍原本与学术事实，严禁凭空篡改或添加未经文献证实的医学断语。
- 遵循知识产权与开源规范，严禁提交未经授权的闭源或侵权资源。

---

## 2. 协作开发流程 (Git Flow & Branching)

本项目采用规范的分支管理策略：

| 分支名 | 说明 |
| :--- | :--- |
| `main` | 生产稳定主分支，仅接受经过充分测试和代码审查的发布合并，禁止直接 push。 |
| `dev` | 日常开发集成主分支，所有特性分支均基于此分支检出并最终合并回此分支。 |
| `feat/*` | 新功能分支（如 `feat/reader-lightbox`, `feat/inverted-index`）。 |
| `fix/*` | 缺陷修复分支（如 `fix/markdown-table-overflow`）。 |
| `docs/*` | 文档更新与补充。 |

### 开发基本步骤
1. **Fork 与克隆**：
   ```bash
   git clone --recurse-submodules https://github.com/<your-username>/TCM-Library-Search.git
   cd TCM-Library-Search
   git checkout -b feat/your-feature-name origin/dev
   ```
2. **保持与子模块同步**：
   ```bash
   git submodule update --remote --merge
   ```
3. **本地开发与自测**：
   确保编写单元测试与必要的视觉回归测试。
4. **提交代码 (Commit)**：
   严格遵循规范提交信息（见下文）。
5. **发起 Pull Request (PR)**：
   提交 PR 至主仓库的 `dev` 分支，详细说明变更动机、改动点与验证方式。

---

## 3. 提交信息规范 (Conventional Commits)

提交信息必须遵循 [Conventional Commits](https://www.conventionalcommits.org/) 规范，格式如下：

```text
<type>(<scope>): <subject>

[optional body]

[optional footer(s)]
```

### Type 类型说明
- `feat`: 新增功能（如检索维度、阅读器字体排版设置、图片预览灯箱）。
- `fix`: 修复问题（如解析 frontmatter 异常、渲染闪烁等）。
- `docs`: 仅文档修改（如 README、接口文档更新）。
- `refactor`: 代码重构（既不修复 bug 也不添加新功能的代码修改）。
- `perf`: 提升性能的修改（如索引加速、内存占用优化）。
- `test`: 新增或修改测试用例。
- `chore`: 杂务（如依赖升级、构建脚本调整、CI 配置）。

### Scope 作用域示例
- `core`: 纯 Rust 检索/计算内核。
- `desktop`: 桌面应用与前端视图。
- `corpus`: 文库与子模块配置。
- `ipc`: Tauri 宿主通道与前后端协议。
- `image`: 图片资源解析与渲染系统。

**示例**：
```bash
git commit -m "feat(core): add multi-field inverted index search for herb categories"
git commit -m "fix(desktop): fix image zoom modal clipping under dark mode"
```

---

## 4. 编码与工程规范

### 4.1 Rust 核心规范 (`crates/core`, `src-tauri`)
- **零 Panic 原则**：生产路径绝对禁止使用 `.unwrap()`, `.expect()`, `panic!()`，一律使用 `Result<T, E>` 显式捕获并向上冒泡。
- **错误类型统一**：统一使用 `thiserror` 定义语义清晰的领域错误枚举。
- **纯净性**：`crates/core` 保持纯算法与数据结构推导，绝不依赖任何 GUI、系统窗口或前端特定的第三方库。
- **代码格式化与 Lint**：
  ```bash
  cargo fmt --all -- --check
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace
  ```

### 4.2 前端与界面规范 (`crates/desktop/src`)
- **TypeScript 严格模式**：禁止使用 `any`，所有 API 返回与组件 Props 必须显式声明类型。
- **响应式状态管理**：充分利用 SolidJS 细粒度响应式特性（`createSignal`, `createMemo`, `createEffect`），避免无谓的全局重新渲染。
- **样式与主题规范**：
  - 一律采用 CSS Modules 或标准 CSS 自定义属性（`var(--bg-primary)`, `var(--text-accent)`）。
  - 支持夜间深色（竹简暗墨）与日间浅色（宣纸仿古）自适应，严禁写死十六进制固定色值。
- **图片与媒体接入**：
  - 遵循 [docs/IMAGE_SPEC.md](docs/IMAGE_SPEC.md) 标准规范，所有图片必须配置 `alt` 描述，且支持加载中骨架屏与断图容错。

---

## 5. 质量门禁与 PR 合并要求

所有 PR 在合并前必须满足以下条件：
1. **CI 检查通过**：自动化流程中的 `cargo fmt`, `cargo clippy`, `cargo test`, `pnpm build` 全部绿灯。
2. **文档对齐**：若涉及 API 接口变更或新功能增补，必须同步更新 [docs/API.md](docs/API.md) 与 [docs/TODO.md](docs/TODO.md)。
3. **无侵权代码**：确认所有引入的外部算法与依赖具有商业友好/宽松开源许可证（如 MIT / Apache-2.0），并妥善注明出处。
4. **代码审查 (Code Review)**：至少获得一位核心维护者的审查认可 (LGTM)。

再次感谢您的投入与智慧，让我们共同铸造兼具典雅书香与现代技术精粹的中医智能文库！
