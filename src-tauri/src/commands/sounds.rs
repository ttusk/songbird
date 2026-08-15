use crate::services::database::{
    sounds::{NewSound, Sound, UpdateSound},
    Database,
};
use tauri::State;

#[tauri::command]
pub fn add_sound(database: State<'_, Database>, sound: NewSound) -> Result<Sound, String> {
    database.add_sound(sound).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn find_sound(database: State<'_, Database>, id: i64) -> Result<Option<Sound>, String> {
    database.find_sound(id).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn list_sounds(database: State<'_, Database>) -> Result<Vec<Sound>, String> {
    database.list_sounds().map_err(|error| error.to_string())
}

#[tauri::command]
pub fn delete_sound(database: State<'_, Database>, id: i64) -> Result<bool, String> {
    database.delete_sound(id).map_err(|error| error.to_string())
}

#[tauri::command]
pub fn update_sound(
    database: State<'_, Database>,
    id: i64,
    sound: UpdateSound,
) -> Result<Option<Sound>, String> {
    database
        .update_sound(id, sound)
        .map_err(|error| error.to_string())
}
