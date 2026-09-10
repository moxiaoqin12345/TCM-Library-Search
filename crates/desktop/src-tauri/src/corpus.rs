use base64::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use tcm_library_core::{
    parse_markdown_entry, search_corpus, CategoryTreeItem, CorpusIndex, CorpusStatus, SearchQuery,
    SearchResultItem, TcmEntryDetail,
};

/// 典籍文库生命周期管理器
pub struct CorpusManager {
    corpus_dir: PathBuf,
    index: Arc<RwLock<CorpusIndex>>,
}

impl CorpusManager {
    pub fn new(corpus_dir: PathBuf) -> Self {
        let mut index = CorpusIndex::default();

        let manifest_path = corpus_dir.join("manifest.json");
        if manifest_path.exists() {
            if let Ok(loaded) = CorpusIndex::from_manifest_file(&manifest_path) {
                index = loaded;
            }
        }

        Self {
            corpus_dir,
            index: Arc::new(RwLock::new(index)),
        }
    }

    /// 后台异步更新/预热索引
    pub fn start_async_indexing(&self) {
        let manifest_path = self.corpus_dir.join("manifest.json");
        let index_lock = Arc::clone(&self.index);

        if manifest_path.exists() {
            std::thread::spawn(move || {
                if let Ok(loaded) = CorpusIndex::from_manifest_file(&manifest_path) {
                    if let Ok(mut lock) = index_lock.write() {
                        *lock = loaded;
                    }
                }
            });
        }
    }

    /// 获取文库总状态
    pub fn get_status(&self) -> CorpusStatus {
        let index = self.index.read().unwrap();
        let total = index.total();
        let cat_count = index.categories.len();

        CorpusStatus {
            total_entries: total,
            total_categories: cat_count,
            is_ready: total > 0,
            root_path: self.corpus_dir.to_string_lossy().to_string(),
        }
    }

    /// 获取分类树
    pub fn list_categories(&self) -> Vec<CategoryTreeItem> {
        let index = self.index.read().unwrap();
        index.build_category_tree()
    }

    /// 多维与模糊检索
    pub fn search(&self, query: &SearchQuery) -> Vec<SearchResultItem> {
        let index = self.index.read().unwrap();
        search_corpus(&index, query)
    }

    /// 读取条目完整三层正文
    pub fn read_entry_detail(&self, id: &str) -> Result<TcmEntryDetail, String> {
        let entry_path = {
            let index = self.index.read().unwrap();
            let entry = index
                .get_by_id(id)
                .ok_or_else(|| format!("未找到 ID 为 '{}' 的条目", id))?;
            self.corpus_dir.join(&entry.path)
        };

        if !entry_path.exists() {
            return Err(format!("条目文件不存在: {:?}", entry_path));
        }

        let content =
            fs::read_to_string(&entry_path).map_err(|e| format!("读取文件失败: {}", e))?;

        parse_markdown_entry(&content).map_err(|e| format!("解析 Markdown 失败: {}", e))
    }

    /// 安全解析图片路径 (杜绝路径穿越)
    pub fn resolve_image_path(&self, id: &str, relative_path: &str) -> Result<PathBuf, String> {
        // 如果是绝对 URL，直接抛出不属于本地解析
        if relative_path.starts_with("http://") || relative_path.starts_with("https://") {
            return Err("远程图片无需本地路径解析".to_string());
        }

        let entry_dir = {
            let index = self.index.read().unwrap();
            let entry = index
                .get_by_id(id)
                .ok_or_else(|| format!("未找到条目 {}", id))?;
            let full_entry_path = self.corpus_dir.join(&entry.path);
            full_entry_path
                .parent()
                .ok_or_else(|| "无法获取上级目录".to_string())?
                .to_path_buf()
        };

        let target_path = entry_dir.join(relative_path);
        let canonical_target = target_path
            .canonicalize()
            .map_err(|e| format!("图片文件不存在或无法访问: {}", e))?;

        let canonical_root = self
            .corpus_dir
            .canonicalize()
            .map_err(|e| format!("文库根目录异常: {}", e))?;

        // 校验沙箱边界：目标路径必须以文库根目录开头
        if !canonical_target.starts_with(&canonical_root) {
            return Err("越界访问拦截：图片路径超出文库沙箱".to_string());
        }

        Ok(canonical_target)
    }

    /// 读取图片为 Base64 Data URI (最可靠跨平台兼容方案)
    pub fn load_image_data_uri(&self, id: &str, relative_path: &str) -> Result<String, String> {
        let abs_path = self.resolve_image_path(id, relative_path)?;
        let bytes = fs::read(&abs_path).map_err(|e| format!("读取图片文件失败: {}", e))?;

        let ext = abs_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_lowercase();

        let mime = match ext.as_str() {
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "webp" => "image/webp",
            "svg" => "image/svg+xml",
            "gif" => "image/gif",
            _ => "application/octet-stream",
        };

        let encoded = BASE64_STANDARD.encode(&bytes);
        Ok(format!("data:{};base64,{}", mime, encoded))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_corpus_manager_on_corpus_dir() {
        let test_dir = Path::new("../../../corpus");
        if !test_dir.exists() {
            return;
        }

        let mgr = CorpusManager::new(test_dir.to_path_buf());
        let status = mgr.get_status();
        assert!(status.total_entries > 0, "Corpus entries should be > 0");

        let categories = mgr.list_categories();
        assert!(!categories.is_empty(), "Categories should not be empty");

        // 检索测试: 桂枝汤
        let q = SearchQuery {
            keyword: Some("桂枝".into()),
            ..Default::default()
        };
        let items = mgr.search(&q);
        assert!(!items.is_empty(), "Should find items matching 桂枝");

        // 详情读取测试
        let first_id = &items[0].id;
        let detail = mgr.read_entry_detail(first_id).unwrap();
        assert!(
            !detail.original_text.is_empty(),
            "Original text should not be empty"
        );
    }
}
