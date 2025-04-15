pub mod app;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            app::greet,
            app::extract_archive_cmd,
            app::compress_archive_cmd
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
