use std::path::PathBuf;
use tauri::Manager;

use crate::services::database::Database;
pub mod commands;
pub mod services;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data_dir)?;

            let database_path: PathBuf = app_data_dir.join("songbird.sqlite3");
            let database = Database::open(database_path)?;

            app.manage(database);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::campaigns::add_campaign,
            commands::campaigns::find_campaign,
            commands::campaigns::list_campaigns,
            commands::campaigns::find_campaign_details,
            commands::campaigns::update_campaign,
            commands::campaigns::delete_campaign,
            commands::characters::find_character,
            commands::characters::update_character,
            commands::characters::delete_character,
            commands::characters::add_character,
            commands::characters::list_characters,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
