mod command;
use tauri::{Window};
#[cfg_attr(mobile, tauri::mobile_entry_point)]

use command::{read_file, write_file, file_exists, download_file, download_file_with_content, delete_file, open_path};
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
           set_window_title,
           read_file,
           write_file,
           delete_file,
           file_exists,
           download_file,
           open_path,
           download_file_with_content,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
fn set_window_title(window: Window, title: String) -> String {
    _ = window.set_title(title.as_str());
    String::from("ok")
}
