use crate::services::database::{
    characters::{Character, NewCharacter},
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
pub fn list_characters(
    database: State<'_, Database>,
    campaign_id: i64,
) -> Result<Vec<Character>, String> {
    database
        .list_characters(campaign_id)
        .map_err(|error| error.to_string())
}
