# TCM-Library-Search 典籍图片与多媒体资源接入规范 (IMAGE_SPEC)

## 1. 概述与设计背景

中医典籍具有极强的图文互证传统：从《本草图经》《金匮要略》到现代针灸学，本草饮片形态、舌苔润燥色泽、经穴寸度走向以及古刻本原貌（书影）均依赖图像辅助研判。

后续 `corpus` 知识库将持续接入多类型医学图像。本规范规定了图片从存储组织、元数据标引、安全加载到前端交互渲染的完整技术标准。

---

## 2. 图像分类与场景定义

| 分类标识 (`type`) | 典型场景 | 推荐格式 | 交互需求 |
| :--- | :--- | :--- | :--- |
| `herb_specimen` | 中药原植物、原动物、饮片性状图谱 | WebP, PNG, JPG | 高清缩放、微距形态观察 |
| `acupoint_meridian` | 十二经络循行路线、奇经八脉、穴位骨度折量图 | SVG, WebP, PNG | 矢量不失真缩放、穴位标注可读 |
| `tongue_pulse` | 中医舌象四诊图解（舌质、舌苔、齿痕、点刺） | JPG, WebP | 真彩保真度、防暗光失真 |
| `book_folio` | 善本古籍刻本书影、名医手迹医案影印件 | WebP, JPG | 古色墨韵滤镜契合、原版对照 |

---

## 3. 存储路径与标引语法

### 3.1 路径组织规范
在 `corpus/library/` 各级目录下，图片资源建议以同级 `images/` 或条目专有资源目录组织：

```text
corpus/library/zhongyao/anshen/baiziren/
├── INDEX.md
├── baiziren_001.md
└── images/
    ├── baiziren_plant.webp       # 原植物形态图
    └── baiziren_decoction.webp   # 饮片药材图
```

### 3.2 标引方式（双轨支持）

#### 方式一：Markdown 行内标引（正文自然流）
```markdown
**【原文】**
柏子仁为柏科植物侧柏的干燥成熟种仁。
![侧柏原植物与成熟种仁](images/baiziren_plant.webp)
```

#### 方式二：Frontmatter 元数据图谱标引（结构化资产清单）
在条目的 YAML Frontmatter 中扩展 `images` 字段，供前端图集或卡片模式独立解析展示：

```yaml
---
id: "baiziren_001"
book: "中国药典（2025年版）一部"
# ...其他基础元数据
images:
  - path: "images/baiziren_plant.webp"
    title: "侧柏植株与球果"
    type: "herb_specimen"
    source: "国家药典委员会"
  - path: "images/baiziren_decoction.webp"
    title: "柏子仁成熟干燥种仁饮片"
    type: "herb_specimen"
---
```

---

## 4. 安全加载与 Tauri 资产协议 (Asset Protocol)

### 4.1 路径解析与安全沙箱 (Sandboxing)
- 前端向 Tauri 宿主请求条目图片时，宿主需基于条目所在的绝对物理路径解析其相对图片路径。
- **安全沙箱边界**：所有图片物理路径必须经过 `canonicalize()` 规范化，并严格校验其位于 `corpus/` 根目录之内，坚决杜绝 `../` 路径穿越攻击。

### 4.2 统一资源 URL 转换
- 桌面端采用 Tauri v2 标准的 Asset 协议或安全的局部流转换：
  ```typescript
  import { convertFileSrc } from "@tauri-apps/api/core";
  
  // 将后端返回的规范化绝对路径转换为安全 webview 资源链接
  const safeImageUrl = convertFileSrc(absoluteImagePath);
  ```
- 若为远程 URL（以 `http://` 或 `https://` 开头），前端直接按标准外部图片加载，并支持本地持久化缓存策略。

---

## 5. 前端交互与阅读器视觉规范

### 5.1 响应式排版自适应
- 正文中图片最大宽度自适应父容器（`max-width: 100%; height: auto;`），居中对齐。
- 图片下方自动渲染标题/图注说明（Caption），采用小号沉稳字体与次级文本色。

### 5.2 全屏高保真灯箱 (Lightbox)
- **点击放大**：用户点击任何插图均可无缝唤起全屏半透明暗色背景的 Lightbox 模态层。
- **多点触控与滚轮交互**：
  - 支持鼠标滚轮缩放（Zoom In/Out，倍率 0.5x ~ 5.0x）。
  - 支持鼠标拖拽平移（Pan）与双击重置（Reset）。
  - 支持快捷键：`ESC` 退出、`← / →` 切换当前条目图集。

### 5.3 容错与视觉降级 (Graceful Degradation)
- **加载状态**：图片加载过程中展示温润的宣纸微光骨架屏（Skeleton Loader）。
- **断图保护**：若本地图片文件不存在或网络失效，自动显示优雅的中医水墨风格占位图（Placeholder），并标注“图谱整理中”，不破坏正文版面结构。
