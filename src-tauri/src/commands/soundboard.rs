use crate::services::database::{
    soundboard::{NewSoundboard, Soundboard, SoundboardSound, UpdateSoundboard},
    Database,
};
use tauri::State;

#[tauri::command]
pub fn add_soundboard(
    database: State<'_, Database>,
    soundboard: NewSoundboard,
) -> Result<Soundboard, String> {
    database
        .add_soundboard(soundboard)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn find_soundboard(
    database: State<'_, Database>,
    id: i64,
) -> Result<Option<Soundboard>, String> {
    database
        .find_soundboard(id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn list_soundboards(database: State<'_, Database>) -> Result<Vec<Soundboard>, String> {
    database
        .list_soundboards()
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn update_soundboard(
    database: State<'_, Database>,
    id: i64,
    soundboard: UpdateSoundboard,
) -> Result<Option<Soundboard>, String> {
    database
        .update_soundboard(id, soundboard)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn delete_soundboard(database: State<'_, Database>, id: i64) -> Result<bool, String> {
    database
        .delete_soundboard(id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn add_sound_to_soundboard(
    database: State<'_, Database>,
    soundboard_id: i64,
    sound_id: i64,
    position: i64,
) -> Result<SoundboardSound, String> {
    database
        .add_sound_to_soundboard(soundboard_id, sound_id, position)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn list_soundboard_sounds(
    database: State<'_, Database>,
    soundboard_id: i64,
) -> Result<Vec<SoundboardSound>, String> {
    database
        .list_soundboard_sounds(soundboard_id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn remove_sound_from_soundboard(
    database: State<'_, Database>,
    soundboard_id: i64,
    sound_id: i64,
) -> Result<bool, String> {
    database
        .remove_sound_from_soundboard(soundboard_id, sound_id)
        .map_err(|error| error.to_string())
}
