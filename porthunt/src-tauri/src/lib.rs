// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn get_core_status() -> String {
    "Porthunt Rust Core is running".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_core_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}