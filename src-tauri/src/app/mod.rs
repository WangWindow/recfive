mod archieve;

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
