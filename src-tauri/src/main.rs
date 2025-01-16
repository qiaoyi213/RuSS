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
    let file = File::open("/Users/linqiaoyi/allProject/RuSS/src-tauri/feeds/example.xml").unwrap();
    let channel = Channel::read_from(BufReader::new(file)).unwrap();
    //let channel = app_lib::core::rss::getFeedByUrl("https://www.nhk.or.jp/rss/news/cat0.xml".to_string());

    //println!("{}", channel.unwrap().items.len().to_string());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
