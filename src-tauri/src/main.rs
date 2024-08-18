// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use tauri::{WindowEvent};
fn main() {
    
    tauri::Builder::default()
        .on_window_event(|event| {
        // ALT + F4 키 이벤트를 감지하고 막음
        if let WindowEvent::CloseRequested { api, .. } = event.event() {
            // 창이 닫히는 것을 방지합니다.
            api.prevent_close(); 
           }
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
