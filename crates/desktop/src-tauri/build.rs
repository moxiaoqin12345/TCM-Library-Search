fn main() {
    // 确保若 submodule corpus 未拉取或为空目录时，tauri.conf.json 中的 resource glob 不会导致构建中断
    let corpus_dir = std::path::Path::new("../../../corpus");
    if !corpus_dir.exists() {
        let _ = std::fs::create_dir_all(corpus_dir);
        let _ = std::fs::write(corpus_dir.join("placeholder.txt"), "");
    } else if let Ok(mut entries) = std::fs::read_dir(corpus_dir) {
        if entries.next().is_none() {
            let _ = std::fs::write(corpus_dir.join("placeholder.txt"), "");
        }
    }

    tauri_build::build();
}
