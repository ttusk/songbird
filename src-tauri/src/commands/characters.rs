use crate::services::database::{
    characters::{Character, NewCharacter, UpdateCharacter},
    Database,
};
use tauri::State;

#[tauri::command]
pub fn add_character(
    database: State<'_, Database>,
    character: NewCharacter,
) -> Result<Character, String> {
    database
        .add_character(character)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn find_character(database: State<'_, Database>, id: i64) -> Result<Option<Character>, String> {
    database
        .find_character(id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn update_character(
    database: State<'_, Database>,
    id: i64,
    character: UpdateCharacter,
) -> Result<Option<Character>, String> {
    database
        .update_character(id, character)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn delete_character(database: State<'_, Database>, id: i64) -> Result<bool, String> {
    database
        .delete_character(id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn list_characters(
    database: State<'_, Database>,
    campaign_id: i64,
) -> Result<Vec<Character>, String> {
    database
        .list_characters(campaign_id)
        .map_err(|error| error.to_string())
}
