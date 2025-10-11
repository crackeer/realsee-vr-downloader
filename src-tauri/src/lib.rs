#[macro_use]
extern crate rust_box;
use rust_box::tauri::command::{
    http_request::{ http_download_file, http_download_file_v2,
    },
};

use rust_box::tauri::command::file::{
    create_dir, create_file, create_jsonp_file, delete_dir, delete_file, file_exists,
    get_file_content, list_folder, rename_file, write_blob_file, write_file,
};
use rust_box::tauri::command::work::write_rsvr_jsonp_asset;
use rust_box::tauri::command::opener::open_path;
use tauri::{Window};
#[cfg_attr(mobile, tauri::mobile_entry_point)]

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            get_file_content,
            write_file,
            list_folder,
            set_window_title,
            write_blob_file,
            create_dir,
            create_file,
            delete_file,
            delete_dir,
            rename_file,
            file_exists,
            http_download_file,
            http_download_file_v2,
            create_jsonp_file,
            write_rsvr_jsonp_asset,
            open_path,
            is_dev,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
fn set_window_title(window: Window, title: String) -> String {
    _ = window.set_title(title.as_str());
    String::from("ok")
}


#[tauri::command]
fn is_dev() -> bool {
    cfg!(debug_assertions) // 开发环境为 true，生产环境为 false
}
