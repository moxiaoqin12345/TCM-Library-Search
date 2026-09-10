use serde::{Deserialize, Serialize};

/// 中医 11 维检索条件
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CorpusConditions {
    #[serde(default)]
    pub zhengxing: Vec<String>, // 证型 (如: 太阳中风, 阴虚)
    #[serde(default)]
    pub zhifa: Vec<String>, // 治法 (如: 解表散寒, 养心安神)
    #[serde(default)]
    pub bingzheng: Vec<String>, // 病症 (如: 虚烦不眠, 肠燥便秘)
    #[serde(default)]
    pub zhengzhuang: Vec<String>, // 症状 (如: 失眠, 心悸, 便秘)
    #[serde(default)]
    pub fangming: Vec<String>, // 方名 (如: 桂枝汤, 天王补心丹)
    #[serde(default)]
    pub yaoming: Vec<String>, // 药名 (如: 柏子仁, 黄连)
    #[serde(default)]
    pub xuewei: Vec<String>, // 腧穴 (如: 足三里, 合谷)
    #[serde(default)]
    pub jingluo: Vec<String>, // 经络 (如: 手太阴肺经)
    #[serde(default)]
    pub siqi: Vec<String>, // 四气 (如: 寒, 热, 温, 凉, 平)
    #[serde(default)]
    pub wuwei: Vec<String>, // 五味 (如: 辛, 甘, 酸, 苦, 咸)
    #[serde(default)]
    pub guijing: Vec<String>, // 归经 (如: 心经, 肝经, 肾经)
    #[serde(default)]
    pub keywords: Vec<String>, // 开放关键词
}

/// 典籍插图信息
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorpusImage {
    pub path: String,  // 相对路径或远程 URL
    pub title: String, // 图像标题
    #[serde(default)]
    pub caption: Option<String>, // 图注说明
    #[serde(rename = "type", default = "default_image_type")]
    pub image_type: String, // "herb_specimen" | "acupoint_meridian" | "tongue_pulse" | "book_folio" | "inline"
    #[serde(default)]
    pub source: Option<String>, // 出处
}

fn default_image_type() -> String {
    "inline".to_string()
}

/// Markdown Frontmatter 元数据
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorpusMetadata {
    pub id: String,
    pub book: String,
    #[serde(default)]
    pub chapter: String,
    #[serde(default)]
    pub section_title: String,
    #[serde(default)]
    pub source_version: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub dynasty: String,
    #[serde(rename = "type", default)]
    pub item_type: String,
    #[serde(default)]
    pub conditions: CorpusConditions,
    #[serde(default)]
    pub weight: u32,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub images: Vec<CorpusImage>,
}

/// 完整的典籍篇目正文与解析结构
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TcmEntryDetail {
    pub metadata: CorpusMetadata,
    /// 正文原文 (一字不改)
    pub original_text: String,
    /// 古注 / 阐微 / 评注
    pub commentary_text: Option<String>,
    /// 白话提要
    pub summary_text: Option<String>,
    /// 包含的插图资产列表 (合并 Frontmatter 与 Markdown 中的图片)
    pub images: Vec<CorpusImage>,
}

/// manifest.json 中的分类定义
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestSubcategory {
    pub id: String,
    pub name: String,
    pub name_zh: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestCategory {
    pub id: String,
    pub name: String,
    pub name_zh: String,
    #[serde(default)]
    pub subcategories: Vec<ManifestSubcategory>,
}

/// manifest.json 中的条目轻量条目
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestEntry {
    pub id: String,
    pub book: String,
    #[serde(rename = "type", default)]
    pub item_type: String,
    #[serde(default)]
    pub tier: u32,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub subcategory: String,
    pub path: String,
    #[serde(default)]
    pub weight: u32,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub chapter: String,
    #[serde(default)]
    pub conditions: CorpusConditions,
}

/// manifest.json 总清单结构
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CorpusManifest {
    #[serde(default)]
    pub schema_version: f64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub categories: Vec<ManifestCategory>,
    #[serde(default)]
    pub total: usize,
    #[serde(default)]
    pub entries: Vec<ManifestEntry>,
}

/// 检索请求参数
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SearchQuery {
    pub keyword: Option<String>,
    pub category: Option<String>,
    pub subcategory: Option<String>,
    pub zhengxing: Option<String>,
    pub zhifa: Option<String>,
    pub bingzheng: Option<String>,
    pub zhengzhuang: Option<String>,
    pub fangming: Option<String>,
    pub yaoming: Option<String>,
    pub xuewei: Option<String>,
    pub jingluo: Option<String>,
    pub siqi: Option<String>,
    pub wuwei: Option<String>,
    pub guijing: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// 检索结果条目
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResultItem {
    pub id: String,
    pub book: String,
    pub chapter: String,
    pub section_title: String,
    pub category: String,
    pub subcategory: String,
    pub item_type: String,
    pub weight: u32,
    pub matched_dimensions: Vec<String>,
    pub snippet: String,
    pub relative_path: String,
}

/// 分类树统计结构
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryTreeItem {
    pub id: String,
    pub name_zh: String,
    pub count: usize,
    pub subcategories: Vec<SubcategoryTreeItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubcategoryTreeItem {
    pub id: String,
    pub name_zh: String,
    pub count: usize,
}

/// 知识库总状态概览
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorpusStatus {
    pub total_entries: usize,
    pub total_categories: usize,
    pub is_ready: bool,
    pub root_path: String,
}

/// 书籍章节条目项
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BookChapterEntryItem {
    pub id: String,
    pub title: String,
    pub section_title: String,
    pub weight: u32,
    pub category: String,
    pub subcategory: String,
}

/// 书籍章节项
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BookChapterTreeItem {
    pub chapter_name: String,
    pub count: usize,
    pub entries: Vec<BookChapterEntryItem>,
}

/// 书目概览项
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BookSummaryItem {
    pub book_name: String,
    pub entry_count: usize,
    pub chapter_count: usize,
    pub chapters: Vec<BookChapterTreeItem>,
}
