mod archieve;

use tauri::AppHandle;
use tauri::command;
use tauri_plugin_dialog::DialogExt;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn extract_archive_cmd(input: String, output: String) -> Result<(), String> {
    archieve::extract_archive(&input, &output).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn compress_archive_cmd(input: String, output: String) -> Result<(), String> {
    archieve::compress_archive(&input, &output).map_err(|e| e.to_string())
}

#[command]
pub async fn pick_file(app: AppHandle) -> Option<String> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    app.dialog().file().pick_file(move |file| {
        let _ = tx.send(file);
    });
    rx.await.ok().flatten().map(|p| p.to_string())
}

#[command]
pub async fn pick_directory(app: AppHandle) -> Option<String> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    app.dialog().file().pick_folder(move |dir| {
        let _ = tx.send(dir);
    });
    rx.await.ok().flatten().map(|p| p.to_string())
}
