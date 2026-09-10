# TCM-Library-Search 核心 API 契约与接口规范 (API)

本规范定义了 Rust Core 内核、Tauri IPC 宿主通道与前端应用之间的数据交换契约。

---

## 1. 核心数据模型 (Data Models)

### 1.1 `CorpusConditions` (中医 11 维检索维度)
```rust
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CorpusConditions {
    #[serde(default)]
    pub zhengxing: Vec<String>,     // 证型 (如: 太阳中风, 阴虚)
    #[serde(default)]
    pub zhifa: Vec<String>,         // 治法 (如: 解表散寒, 养心安神)
    #[serde(default)]
    pub bingzheng: Vec<String>,     // 病症 (如: 虚烦不眠, 肠燥便秘)
    #[serde(default)]
    pub zhengzhuang: Vec<String>,   // 症状 (如: 失眠, 心悸, 便秘)
    #[serde(default)]
    pub fangming: Vec<String>,      // 方名 (如: 桂枝汤, 天王补心丹)
    #[serde(default)]
    pub yaoming: Vec<String>,       // 药名 (如: 柏子仁, 黄连)
    #[serde(default)]
    pub xuewei: Vec<String>,        // 腧穴 (如: 足三里, 合谷)
    #[serde(default)]
    pub jingluo: Vec<String>,       // 经络 (如: 手太阴肺经)
    #[serde(default)]
    pub siqi: Vec<String>,          // 四气 (如: 寒, 热, 温, 凉, 平)
    #[serde(default)]
    pub wuwei: Vec<String>,         // 五味 (如: 辛, 甘, 酸, 苦, 咸)
    #[serde(default)]
    pub guijing: Vec<String>,       // 归经 (如: 心经, 肝经, 肾经)
    #[serde(default)]
    pub keywords: Vec<String>,      // 开放关键词
}
```

### 1.2 `CorpusImage` (插图资产元数据)
```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorpusImage {
    pub path: String,               // 相对或绝对路径 / 远程 URL
    pub title: String,              // 标题说明
    #[serde(default)]
    pub caption: Option<String>,    // 详细图注
    #[serde(rename = "type", default)]
    pub image_type: String,         // "herb_specimen" | "acupoint_meridian" | "tongue_pulse" | "book_folio"
    #[serde(default)]
    pub source: Option<String>,     // 来源出处
}
```

### 1.3 `TcmEntrySummary` (轻量检索摘要，供列表快速展示)
```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TcmEntrySummary {
    pub id: String,
    pub book: String,
    pub chapter: String,
    pub section_title: String,
    pub dynasty: String,
    pub author: String,
    pub item_type: String,
    pub weight: u32,
    pub tags: Vec<String>,
    pub matched_dimensions: Vec<String>,  // 命中哪些维度
    pub snippet: String,                  // 摘要预览文本
    pub thumbnail: Option<String>,        // 首图缩略图
}
```

### 1.4 `TcmEntryDetail` (条目完整内容，供阅读器精读)
```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TcmEntryDetail {
    pub metadata: CorpusMetadata,
    pub original_text: String,            // 【原文】原汁原味
    pub commentary_text: Option<String>,  // 【古注】名家阐微
    pub summary_text: Option<String>,     // 【白话提要】白话提要与现代药理
    pub images: Vec<CorpusImage>,         // 包含的所有图谱插图
}
```

---

## 2. Tauri IPC Commands

| 命令名 (`invoke`) | 参数 | 返回值 | 说明 |
| :--- | :--- | :--- | :--- |
| `get_corpus_status` | 无 | `CorpusStatus` | 获取当前已索引典籍总数、分类统计与就绪状态 |
| `search_entries` | `req: SearchRequest` | `Vec<TcmEntrySummary>` | 多维复合与关键词检索，带分页与排序 |
| `get_entry_detail` | `id: String` | `Option<TcmEntryDetail>` | 获取指定典籍篇目的三层正文与图片资源 |
| `list_categories` | 无 | `Vec<CategoryTreeItem>` | 获取 10 大分类与 47 个子分类的层级树 |
| `resolve_image_path` | `id: String, relative_path: String` | `String` | 安全解析条目插图在宿主中的可访问安全协议 URL |

### 示例：`search_entries` 入参结构
```typescript
interface SearchRequest {
  keyword?: string;               // 模糊关键词
  category?: string;              // 顶层大类过滤 (如 "zhongyao")
  sub_category?: string;          // 子类过滤 (如 "anshen")
  conditions?: Partial<CorpusConditions>; // 11 维精准条件
  limit?: number;                 // 每页条数 (默认 50)
  offset?: number;                // 偏移量
}
```

---

## 3. 错误码规范

所有 IPC 调用发生异常时，统一由统一错误信封承载：

```typescript
interface AppError {
  code: "NOT_FOUND" | "PARSE_ERROR" | "INDEX_UNREADY" | "SECURITY_DENIED" | "INTERNAL";
  message: string;
}
```
