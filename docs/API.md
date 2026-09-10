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

### 1.5 `BookSummaryItem` 与篇章目录树模型
```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BookChapterEntryItem {
    pub id: String,
    pub title: String,
    pub section_title: String,
    pub weight: u32,
    pub category: String,
    pub subcategory: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BookChapterTreeItem {
    pub chapter_name: String,
    pub count: usize,
    pub entries: Vec<BookChapterEntryItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BookSummaryItem {
    pub book_name: String,
    pub entry_count: usize,
    pub chapter_count: usize,
    pub chapters: Vec<BookChapterTreeItem>,
}
```

### 1.6 `CompatibilityAlert` (中药配伍禁忌预警模型 · M1)
```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncompatibilityType {
    #[serde(rename = "shiba_fan")]
    ShibaFan,               // 十八反
    #[serde(rename = "shijiu_wei")]
    ShijiuWei,              // 十九畏
    #[serde(rename = "renshen_jinji")]
    RenshenJinji,           // 妊娠禁忌 (慎用/禁用)
    #[serde(rename = "xiang_e")]
    XiangE,                 // 相恶相杀 (扩展)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncompatibilitySeverity {
    #[serde(rename = "severe")]
    Severe,                 // 严厉禁忌 (十八反、妊娠禁用)
    #[serde(rename = "warning")]
    Warning,                // 警戒提示 (十九畏、妊娠慎用)
    #[serde(rename = "info")]
    Info,                   // 参考提示
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityAlert {
    pub herb_a: String,
    pub herb_b: Option<String>,
    pub incompatibility_type: IncompatibilityType,
    pub severity: IncompatibilitySeverity,
    pub source_rhyme: String,   // 经典歌诀出处
    pub explanation: String,    // 名家考辨与药理机制
}
```

### 1.7 `TextDiffResult` (古今双篇异文对比模型 · M2)
```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiffOp {
    #[serde(rename = "equal")]
    Equal,                  // 未变异文
    #[serde(rename = "delete")]
    Delete,                 // 传抄脱漏/基础本特有
    #[serde(rename = "insert")]
    Insert,                 // 衍文/化裁本新增
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffChunk {
    pub op: DiffOp,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextDiffResult {
    pub chunks: Vec<DiffChunk>,
    pub text_a: String,
    pub text_b: String,
    pub similarity: f64,    // 文本相似度 (0.0 ~ 1.0)
    pub equal_chars: usize,
    pub deleted_chars: usize,
    pub inserted_chars: usize,
}
```

### 1.8 `MeridianInfo` 与 `AcupointInfo` (经络循行与特定腧穴模型 · M3)
```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AcupointInfo {
    pub name: String,
    pub code: String,                   // 国际编码 (如 LU9, ST36)
    pub meridian_name: String,          // 归经 (如 手太阴肺经)
    pub location: String,               // 准确解剖取穴定位
    pub origin_classic: String,         // 古籍出处 (如 《针灸甲乙经》)
    pub specific_types: Vec<String>,    // 特定穴类别 (五输穴、原穴、络穴等)
    pub specific_tags: Vec<String>,     // 快速标签 (原穴, 输穴, 脉会等)
    pub indications: Vec<String>,       // 临床主治证候
    pub manipulation: String,           // 刺灸法度与禁忌
    pub flow_position: usize,           // 经络循行顺位次序
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MeridianInfo {
    pub name: String,
    pub code: String,
    pub category: String,               // shou_san_yin, zu_san_yang 等
    pub element: String,                // 五行归属 (金木水火土)
    pub paired_meridian: String,        // 表里经配偶
    pub peak_time: String,              // 子午流注气血旺衰时辰 (如 寅时 03:00-05:00)
    pub course_description: String,     // 经脉循行径路
    pub acupoints: Vec<AcupointInfo>,   // 本经所辖腧穴列表
}
```

---

## 2. Tauri IPC Commands

| 命令名 (`invoke`) | 参数 | 返回值 | 说明 |
| :--- | :--- | :--- | :--- |
| `get_corpus_status` | 无 | `CorpusStatus` | 获取当前已索引典籍总数、分类统计与就绪状态 |
| `search_entries` | `query: SearchQuery` | `Vec<SearchResultItem>` | 多维复合与关键词检索，带分页与排序 |
| `get_entry_detail` | `id: String` | `Option<TcmEntryDetail>` | 获取指定典籍篇目的三层正文与图片资源 |
| `list_categories` | 无 | `Vec<CategoryTreeItem>` | 获取 10 大分类与 47 个子分类的层级树 |
| `list_books` | 无 | `Vec<BookSummaryItem>` | 获取典籍全书列表与篇章总数统计 |
| `get_book_chapters` | `book: String` | `Option<BookSummaryItem>` | 获取单部典籍的详细章节目次树 |
| `resolve_image_path` | `id: String, relative_path: String` | `String` | 安全解析条目插图在宿主中的可访问安全协议 URL |
| `load_image_data_uri` | `id: String, relative_path: String` | `String` | 读取插图并转为安全的 Base64 Data URI |
| `check_compatibility` | `herbs: Vec<String>` | `Vec<CompatibilityAlert>` | **(M1)** 十八反、十九畏与妊娠用药禁忌实时检测 |
| `diff_text_versions` | `text_a: String, text_b: String` | `TextDiffResult` | **(M2)** LCS 算法古今医籍异文版本多粒度比对 |
| `list_meridians` | 无 | `Vec<MeridianInfo>` | **(M3)** 获取十二经脉气血循行流注全谱与特定穴 |
| `get_acupoint_detail` | `name: String` | `Option<AcupointInfo>` | **(M3)** 精准获取腧穴解剖定位、主治与刺灸法 |
| `recommend_acupoints` | `symptom: String` | `Vec<AcupointInfo>` | **(M3)** 基于临床病症关键词推导推荐辨证特定要穴 |

### 示例：`search_entries` 入参结构
```typescript
interface SearchQuery {
  keyword?: string;               // 模糊关键词
  category?: string;              // 顶层大类过滤 (如 "zhongyao")
  subcategory?: string;           // 子类过滤 (如 "anshen")
  zhengxing?: string;             // 证型
  zhifa?: string;                 // 治法
  bingzheng?: string;             // 病症
  zhengzhuang?: string;           // 症状
  fangming?: string;              // 方名
  yaoming?: string;               // 药名
  xuewei?: string;                // 腧穴
  jingluo?: string;               // 经络
  siqi?: string;                  // 四气
  wuwei?: string;                 // 五味
  guijing?: string;               // 归经
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
