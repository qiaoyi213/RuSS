pub mod core;

use tauri::{
    menu::{AboutMetadata, MenuBuilder, MenuItemBuilder, SubmenuBuilder},
    Emitter,
};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let settings = MenuItemBuilder::new("Settings...")
                .id("settings")
                .accelerator("CmdOrCtrl+,")
                .build(app)?;

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

            let menu = MenuBuilder::new(app).items(&[&app_submenu]).build()?;
            app.set_menu(menu)?;

            app.on_menu_event(move |app, event| {
                if event.id() == settings.id() {
                    let _ = app.emit("settings", "/settings");
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
            core::rss::example_feed,
            core::rss::getSources,
            core::rss::addSource,
            core::rss::getSourceInfo,
            core::rss::getFeed,
            core::rss::deleteSource
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
