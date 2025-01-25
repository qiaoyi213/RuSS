// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use rss::Channel;
use std::io::BufReader;
use std::fs::File;
use app_lib::core;
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, app_lib::core::rss::example_feed])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
