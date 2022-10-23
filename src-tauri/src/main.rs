#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::dispaly::{a_display, reset, count_steps,exsit};

mod dispaly;
mod cmd;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tauri::command]
fn test() {}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, a_display, test,reset,count_steps,exsit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
