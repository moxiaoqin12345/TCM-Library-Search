use crate::error::{CoreError, Result};
use crate::models::{
    BookChapterEntryItem, BookChapterTreeItem, BookSummaryItem, CategoryTreeItem, CorpusManifest,
    ManifestCategory, ManifestEntry, SubcategoryTreeItem,
};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// 内存索引与元数据管理中心
#[derive(Debug, Clone, Default)]
pub struct CorpusIndex {
    pub manifest_name: String,
    pub categories: Vec<ManifestCategory>,
    pub entries: Vec<ManifestEntry>,
    id_to_index: HashMap<String, usize>,
    category_counts: HashMap<String, usize>,
    subcategory_counts: HashMap<String, usize>,
}

impl CorpusIndex {
    /// 从 manifest.json 文件直接快速载入 (毫秒级)
    pub fn from_manifest_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())?;
        Self::from_manifest_str(&content)
    }

    /// 从 JSON 文本解析
    pub fn from_manifest_str(json_str: &str) -> Result<Self> {
        let manifest: CorpusManifest = serde_json::from_str(json_str).map_err(CoreError::Json)?;

        let mut id_to_index = HashMap::new();
        let mut category_counts = HashMap::new();
        let mut subcategory_counts = HashMap::new();

        for (idx, entry) in manifest.entries.iter().enumerate() {
            id_to_index.insert(entry.id.clone(), idx);

            if !entry.category.is_empty() {
                *category_counts.entry(entry.category.clone()).or_insert(0) += 1;
            }
            if !entry.subcategory.is_empty() {
                let key = format!("{}/{}", entry.category, entry.subcategory);
                *subcategory_counts.entry(key).or_insert(0) += 1;
            }
        }

        Ok(Self {
            manifest_name: manifest.name,
            categories: manifest.categories,
            entries: manifest.entries,
            id_to_index,
            category_counts,
            subcategory_counts,
        })
    }

    pub fn total(&self) -> usize {
        self.entries.len()
    }

    pub fn get_by_id(&self, id: &str) -> Option<&ManifestEntry> {
        self.id_to_index
            .get(id)
            .and_then(|&idx| self.entries.get(idx))
    }

    pub fn all_entries(&self) -> &[ManifestEntry] {
        &self.entries
    }

    /// 生成带有条目数量统计的分类树
    pub fn build_category_tree(&self) -> Vec<CategoryTreeItem> {
        let mut tree = Vec::new();

        for cat in &self.categories {
            let cat_count = self.category_counts.get(&cat.id).copied().unwrap_or(0);
            let mut subcategories = Vec::new();

            for sub in &cat.subcategories {
                let key = format!("{}/{}", cat.id, sub.id);
                let sub_count = self.subcategory_counts.get(&key).copied().unwrap_or(0);
                subcategories.push(SubcategoryTreeItem {
                    id: sub.id.clone(),
                    name_zh: sub.name_zh.clone(),
                    count: sub_count,
                });
            }

            tree.push(CategoryTreeItem {
                id: cat.id.clone(),
                name_zh: cat.name_zh.clone(),
                count: cat_count,
                subcategories,
            });
        }

        tree
    }

    /// 获取所有典籍书目及其卷次篇章树
    pub fn list_books(&self) -> Vec<BookSummaryItem> {
        type ChapterEntriesMap = HashMap<String, Vec<BookChapterEntryItem>>;
        type BookChapterData = (Vec<String>, ChapterEntriesMap);

        // 维持稳定的书目顺序
        let mut book_order = Vec::new();
        // book -> (chapter_order, chapters)
        let mut book_map: HashMap<String, BookChapterData> = HashMap::new();

        for entry in &self.entries {
            let book = if entry.book.is_empty() {
                "其他典籍".to_string()
            } else {
                entry.book.clone()
            };

            let chapter = if entry.chapter.is_empty() {
                "未分卷".to_string()
            } else {
                entry.chapter.clone()
            };

            let (chapter_order, chapters) = book_map.entry(book.clone()).or_insert_with(|| {
                book_order.push(book.clone());
                (Vec::new(), HashMap::new())
            });

            let entries = chapters.entry(chapter.clone()).or_insert_with(|| {
                chapter_order.push(chapter.clone());
                Vec::new()
            });

            entries.push(BookChapterEntryItem {
                id: entry.id.clone(),
                title: entry.title.clone(),
                section_title: if entry.title.is_empty() {
                    entry.id.clone()
                } else {
                    entry.title.clone()
                },
                weight: entry.weight,
                category: entry.category.clone(),
                subcategory: entry.subcategory.clone(),
            });
        }

        let mut result = Vec::new();
        for book_name in book_order {
            if let Some((chapter_order, mut chapters)) = book_map.remove(&book_name) {
                let mut total_entries = 0;
                let mut chapter_items = Vec::new();

                for ch_name in chapter_order {
                    if let Some(entries) = chapters.remove(&ch_name) {
                        let count = entries.len();
                        total_entries += count;
                        chapter_items.push(BookChapterTreeItem {
                            chapter_name: ch_name,
                            count,
                            entries,
                        });
                    }
                }

                result.push(BookSummaryItem {
                    book_name,
                    entry_count: total_entries,
                    chapter_count: chapter_items.len(),
                    chapters: chapter_items,
                });
            }
        }

        result
    }

    /// 查询特定书目的章节树
    pub fn get_book_chapter_tree(&self, target_book: &str) -> Option<BookSummaryItem> {
        let books = self.list_books();
        books
            .into_iter()
            .find(|b| b.book_name == target_book || b.book_name.contains(target_book))
    }
}

/// 扫描目录定位 manifest.json 或 corpus 根目录
pub fn locate_manifest_path(start_dir: &Path) -> Option<PathBuf> {
    let candidates = [
        start_dir.join("manifest.json"),
        start_dir.join("corpus").join("manifest.json"),
        start_dir.join("..").join("corpus").join("manifest.json"),
        start_dir
            .join("..")
            .join("..")
            .join("corpus")
            .join("manifest.json"),
        start_dir
            .join("..")
            .join("..")
            .join("..")
            .join("corpus")
            .join("manifest.json"),
    ];

    for candidate in &candidates {
        if candidate.exists() {
            return candidate.canonicalize().ok();
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_build_from_mini_json() {
        let json = r#"{
            "schema_version": 2.1,
            "name": "Test Library",
            "categories": [
                {
                    "id": "zhongyao",
                    "name": "zhongyao",
                    "name_zh": "中药学",
                    "subcategories": [
                        { "id": "anshen", "name": "anshen", "name_zh": "安神药" }
                    ]
                }
            ],
            "total": 1,
            "entries": [
                {
                    "id": "baiziren_001",
                    "book": "药典",
                    "category": "zhongyao",
                    "subcategory": "anshen",
                    "path": "library/zhongyao/anshen/baiziren_001.md",
                    "title": "柏子仁",
                    "weight": 8,
                    "conditions": {
                        "yaoming": ["柏子仁"]
                    }
                }
            ]
        }"#;

        let index = CorpusIndex::from_manifest_str(json).unwrap();
        assert_eq!(index.total(), 1);
        let entry = index.get_by_id("baiziren_001").unwrap();
        assert_eq!(entry.title, "柏子仁");

        let tree = index.build_category_tree();
        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].count, 1);
        assert_eq!(tree[0].subcategories[0].count, 1);

        let books = index.list_books();
        assert_eq!(books.len(), 1);
        assert_eq!(books[0].book_name, "药典");
        assert_eq!(books[0].entry_count, 1);

        let chapter_tree = index.get_book_chapter_tree("药典");
        assert!(chapter_tree.is_some());
    }
}
