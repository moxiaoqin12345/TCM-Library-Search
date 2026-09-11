pub mod corpus;

use corpus::CorpusManager;
use std::path::PathBuf;
use tauri::{Manager, State};
use tcm_library_core::{
    check_herb_compatibility, diff_texts, find_acupoint, get_meridian_knowledge_base,
    locate_manifest_path, recommend_acupoints_for_symptom, AcupointInfo, BookSummaryItem,
    CategoryTreeItem, CompatibilityAlert, CorpusStatus, MeridianInfo, SearchQuery,
    SearchResultItem, TcmEntryDetail, TextDiffResult,
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

/// 重启应用（用于更新后重启生效）
#[tauri::command]
fn app_restart(app: tauri::AppHandle) {
    app.restart();
}

/// 获取应用构建版本号
#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
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

/// 药对配伍禁忌碰撞检测（十八反、十九畏、妊娠用药禁忌）
#[tauri::command]
fn check_compatibility(herbs: Vec<String>) -> Vec<CompatibilityAlert> {
    check_herb_compatibility(&herbs)
}

/// 古今双篇典籍正文异文互校对比
#[tauri::command]
fn diff_text_versions(text_a: String, text_b: String) -> TextDiffResult {
    diff_texts(&text_a, &text_b)
}

/// 获取全量十四经脉与重点穴位知识库
#[tauri::command]
fn list_meridians() -> Vec<MeridianInfo> {
    get_meridian_knowledge_base()
}

/// 穴位临床速查与定位辨析
#[tauri::command]
fn get_acupoint_detail(name: String) -> Option<AcupointInfo> {
    find_acupoint(&name)
}

/// 依据临床病症智能推荐对偶配穴处方
#[tauri::command]
fn recommend_acupoints(symptom: String) -> Vec<AcupointInfo> {
    recommend_acupoints_for_symptom(&symptom)
}

fn resolve_corpus_dir(app: &tauri::App) -> PathBuf {
    // 1. 尝试从应用资源目录检索
    if let Ok(resource_dir) = app.path().resource_dir() {
        if let Some(manifest) = locate_manifest_path(&resource_dir) {
            if let Some(parent) = manifest.parent() {
                return parent.to_path_buf();
            }
        }
    }

    // 2. 尝试从当前可执行文件所在目录检索（涵盖直接运行或安装目录）
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            if let Some(manifest) = locate_manifest_path(exe_dir) {
                if let Some(parent) = manifest.parent() {
                    return parent.to_path_buf();
                }
            }
        }
    }

    // 3. 开发环境相对路径回退检索
    let search_paths = [
        PathBuf::from("../../../corpus"),
        PathBuf::from("../../corpus"),
        PathBuf::from("../corpus"),
        PathBuf::from("corpus"),
    ];

    for path in &search_paths {
        if path.join("manifest.json").exists() || path.exists() {
            return path.canonicalize().unwrap_or_else(|_| path.clone());
        }
    }

    PathBuf::from("../../../corpus")
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
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
            app_restart,
            get_app_version,
            get_corpus_status,
            list_categories,
            list_books,
            get_book_chapters,
            search_entries,
            get_entry_detail,
            resolve_image_path,
            load_image_data_uri,
            check_compatibility,
            diff_text_versions,
            list_meridians,
            get_acupoint_detail,
            recommend_acupoints,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
