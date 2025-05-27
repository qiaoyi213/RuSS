// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use app_lib::core;
use rss::Channel;
use std::fs::File;
use std::io::BufReader;
use tauri::{ 
    menu:: {AboutMetadata,MenuBuilder, MenuItemBuilder, SubmenuBuilder},
    Manager,
    Emitter
};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app|  {
            let handle = app.handle();
            let settings = MenuItemBuilder::new("Settings...")
                .id("settings")
                .accelerator("CmdOrCtrl+,")
                .build(app)?;

            // my custom app submenu
            let app_submenu = SubmenuBuilder::new(app, "App")
                .about(Some(AboutMetadata {
                    ..Default::default()
                }))
                .separator()
                .item(&settings)
                .separator()
                .services()
                .separator()
                .hide()
                .hide_others()
                .quit()
                .build()?; 

            let menu = MenuBuilder::new(app)
                .items(&[
                    &app_submenu,
                ])
                .build()?;

            app.set_menu(menu)?;

            app.on_menu_event(move |app, event| {
                if event.id() == settings.id() {
                    let _event = app.emit("settings", "/settings");
                    println!("Click");
                }
            });
            Ok(())
        })
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
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
