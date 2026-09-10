pub mod corpus;

use corpus::CorpusManager;
use std::path::PathBuf;
use tauri::{Manager, State};
use tcm_library_core::{
    BookSummaryItem, CategoryTreeItem, CorpusStatus, SearchQuery, SearchResultItem, TcmEntryDetail,
};

/// 健康检查
#[tauri::command]
fn check_health() -> String {
    "TCM-Library-Search engine operational".to_string()
}

/// 窗口最小化
#[tauri::command]
fn app_window_minimize(window: tauri::Window) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())
}

/// 窗口最大化/还原切换
#[tauri::command]
fn app_window_toggle_maximize(window: tauri::Window) -> Result<bool, String> {
    let is_max = window.is_maximized().map_err(|e| e.to_string())?;
    if is_max {
        window.unmaximize().map_err(|e| e.to_string())?;
        Ok(false)
    } else {
        window.maximize().map_err(|e| e.to_string())?;
        Ok(true)
    }
}

/// 查询窗口是否最大化
#[tauri::command]
fn app_window_is_maximized(window: tauri::Window) -> Result<bool, String> {
    window.is_maximized().map_err(|e| e.to_string())
}

/// 窗口关闭
#[tauri::command]
fn app_window_close(window: tauri::Window) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}

/// 窗口拖拽
#[tauri::command]
fn app_window_start_dragging(window: tauri::Window) -> Result<(), String> {
    window.start_dragging().map_err(|e| e.to_string())
}

/// 获取文库总状态
#[tauri::command]
fn get_corpus_status(manager: State<CorpusManager>) -> CorpusStatus {
    manager.get_status()
}

/// 获取全部分类树
#[tauri::command]
fn list_categories(manager: State<CorpusManager>) -> Vec<CategoryTreeItem> {
    manager.list_categories()
}

/// 获取全量典籍书目及其篇章树
#[tauri::command]
fn list_books(manager: State<CorpusManager>) -> Vec<BookSummaryItem> {
    manager.list_books()
}

/// 获取指定典籍的卷次章节结构
#[tauri::command]
fn get_book_chapters(
    manager: State<CorpusManager>,
    book: String,
) -> Result<BookSummaryItem, String> {
    manager.get_book_chapters(&book)
}

/// 多维与关键词综合检索
#[tauri::command]
fn search_entries(manager: State<CorpusManager>, query: SearchQuery) -> Vec<SearchResultItem> {
    manager.search(&query)
}

/// 获取条目详细三层正文
#[tauri::command]
fn get_entry_detail(manager: State<CorpusManager>, id: String) -> Result<TcmEntryDetail, String> {
    manager.read_entry_detail(&id)
}

/// 安全解析图片绝对路径
#[tauri::command]
fn resolve_image_path(
    manager: State<CorpusManager>,
    id: String,
    relative_path: String,
) -> Result<String, String> {
    manager
        .resolve_image_path(&id, &relative_path)
        .map(|p| p.to_string_lossy().to_string())
}

/// 加载图片为 Base64 Data URI
#[tauri::command]
fn load_image_data_uri(
    manager: State<CorpusManager>,
    id: String,
    relative_path: String,
) -> Result<String, String> {
    manager.load_image_data_uri(&id, &relative_path)
}

fn resolve_corpus_dir(app: &tauri::App) -> PathBuf {
    if let Ok(resource_dir) = app.path().resource_dir() {
        let bundled = resource_dir.join("corpus");
        if bundled.exists() {
            return bundled;
        }
    }

    let search_paths = [
        PathBuf::from("../../../corpus"),
        PathBuf::from("../../corpus"),
        PathBuf::from("../corpus"),
        PathBuf::from("corpus"),
    ];

    for path in &search_paths {
        if path.exists() {
            return path.canonicalize().unwrap_or_else(|_| path.clone());
        }
    }

    PathBuf::from("../../../corpus")
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let corpus_dir = resolve_corpus_dir(app);
            let corpus_manager = CorpusManager::new(corpus_dir);
            corpus_manager.start_async_indexing();
            app.manage(corpus_manager);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            check_health,
            app_window_minimize,
            app_window_toggle_maximize,
            app_window_is_maximized,
            app_window_close,
            app_window_start_dragging,
            get_corpus_status,
            list_categories,
            list_books,
            get_book_chapters,
            search_entries,
            get_entry_detail,
            resolve_image_path,
            load_image_data_uri,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
