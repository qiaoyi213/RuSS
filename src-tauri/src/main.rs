// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use app_lib::core;
use rss::Channel;
use std::fs::File;
use std::io::BufReader;
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            app_lib::core::rss::example_feed,
            app_lib::core::rss::getSources,
            app_lib::core::rss::addSource,
            app_lib::core::rss::getSourceInfo,
            app_lib::core::rss::getFeed
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
