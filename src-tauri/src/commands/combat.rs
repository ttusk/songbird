use crate::services::database::{
    combat::{
        CombatParticipant, CombatSession, NewCombatParticipant, NewCombatSession,
        UpdateCombatParticipant, UpdateCombatSession,
    },
    Database,
};
use tauri::State;

#[tauri::command]
pub fn add_combat_session(
    database: State<'_, Database>,
    session: NewCombatSession,
) -> Result<CombatSession, String> {
    database
        .add_combat_session(session)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn find_combat_session(
    database: State<'_, Database>,
    id: i64,
) -> Result<Option<CombatSession>, String> {
    database
        .find_combat_session(id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn list_combat_sessions(
    database: State<'_, Database>,
    campaign_id: i64,
) -> Result<Vec<CombatSession>, String> {
    database
        .list_combat_sessions(campaign_id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn update_combat_session(
    database: State<'_, Database>,
    id: i64,
    session: UpdateCombatSession,
) -> Result<Option<CombatSession>, String> {
    database
        .update_combat_session(id, session)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn delete_combat_session(database: State<'_, Database>, id: i64) -> Result<bool, String> {
    database
        .delete_combat_session(id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn add_combat_participant(
    database: State<'_, Database>,
    participant: NewCombatParticipant,
) -> Result<CombatParticipant, String> {
    database
        .add_combat_participant(participant)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn find_combat_participant(
    database: State<'_, Database>,
    id: i64,
) -> Result<Option<CombatParticipant>, String> {
    database
        .find_combat_participant(id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn list_combat_participants(
    database: State<'_, Database>,
    combat_session_id: i64,
) -> Result<Vec<CombatParticipant>, String> {
    database
        .list_combat_participants(combat_session_id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn update_combat_participant(
    database: State<'_, Database>,
    id: i64,
    participant: UpdateCombatParticipant,
) -> Result<Option<CombatParticipant>, String> {
    database
        .update_combat_participant(id, participant)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn delete_combat_participant(database: State<'_, Database>, id: i64) -> Result<bool, String> {
    database
        .delete_combat_participant(id)
        .map_err(|error| error.to_string())
}
