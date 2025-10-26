use serde_json::{json, Value};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};
use reqwest;
use open;

#[tauri::command]
pub fn write_file(file_path: String, content: String) -> Result<(), String> {
    let mut file = File::create(file_path).map_err(|e| e.to_string())?;
    file.write(content.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_file(file_path: String) -> Result<(), String> {
   fs::remove_file(String::from(file_path)).map_err(|e| e.to_string())?;
   Ok(())
}

#[tauri::command]
pub fn file_exists(file_path: String) -> Result<Value, ()> {
    Ok(json!({
        "exists" : Path::new(&file_path).exists(),
    }))
}

#[tauri::command]
pub fn read_file(file_path: String) -> Result<String, String> {
    let mut file = File::open(file_path).map_err(|e| e.to_string())?;
    let mut content = String::new();
    file.read_to_string(&mut content).map_err(|e| e.to_string())?;
    Ok(content)
}

#[allow(dead_code)]
pub fn create_parent_directory<P>(path : P) -> Result<(), String> 
where P: AsRef<str> {
    let path = Path::new(path.as_ref());
    let path = path.parent().ok_or("no parent")?;
    std::fs::create_dir_all(path).map_err(|err| err.to_string())?;
    return Ok(());
}

pub async fn download_file_to(url: &str, dest : &str) ->  Result<(), String> {
    match reqwest::get(url).await  {
        Ok(res) => {
            let bytes = res.bytes().await;
            let content = bytes.unwrap().as_ref().to_vec();
            if let Err(err) = fs::write(Path::new(dest), &content) {
                return Err(err.to_string());
            }
            Ok(())
        }
        Err(err) => Err(err.to_string())
    }
}

pub async fn download_file_to_v2(url: &str, dest : &str) -> Result<String, String> {
    match reqwest::get(url).await  {
        Ok(res) => {
            let bytes = res.bytes().await;
            let content = bytes.unwrap().as_ref().to_vec();
            if let Err(err) = fs::write(Path::new(dest), &content) {
                return Err(err.to_string());
            }
            Ok(String::from_utf8_lossy(&content).to_string())
        }
        Err(err) => Err(err.to_string())
    }
}

#[tauri::command]
pub async fn download_file(url: String, save_path: String) -> Result<(), String> {
    create_parent_directory(save_path.as_str())?;
    download_file_to(url.as_str(), save_path.as_str()).await?;
    Ok(())
}

#[tauri::command]
pub async fn download_file_with_content(url: String, save_path: String) -> Result<String, String> {
    create_parent_directory(save_path.as_str())?;
    download_file_to_v2(url.as_str(), save_path.as_str()).await
}

#[tauri::command]
pub async fn open_path(file_path: String) -> Result<(), String> {
    open::that(&file_path).map_err(|e| e.to_string())
}