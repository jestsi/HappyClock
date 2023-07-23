// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use chrono::Local;
#[tauri::command(async)]
async fn get_now_time() -> String {
    Local::now().time().format("%H:%M:%S").to_string()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_now_time])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
