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
            commands::sounds::add_sound,
            commands::sounds::find_sound,
            commands::sounds::list_sounds,
            commands::sounds::update_sound,
            commands::sounds::delete_sound,
            commands::soundboard::add_soundboard,
            commands::soundboard::find_soundboard,
            commands::soundboard::list_soundboards,
            commands::soundboard::update_soundboard,
            commands::soundboard::delete_soundboard,
            commands::soundboard::add_sound_to_soundboard,
            commands::soundboard::list_soundboard_sounds,
            commands::soundboard::remove_sound_from_soundboard,
            commands::combat::add_combat_session,
            commands::combat::find_combat_session,
            commands::combat::list_combat_sessions,
            commands::combat::update_combat_session,
            commands::combat::delete_combat_session,
            commands::combat::add_combat_participant,
            commands::combat::find_combat_participant,
            commands::combat::list_combat_participants,
            commands::combat::update_combat_participant,
            commands::combat::delete_combat_participant,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
