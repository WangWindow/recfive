pub mod app;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            app::greet,
            app::extract_archive_cmd,
            app::compress_archive_cmd,
            app::pick_file,
            app::pick_directory
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
