use serde_json::json;
use songbird_lib::services::database::{
    campaign::{Campaign, NewCampaign},
    characters::NewCharacter,
    combat::{NewCombatParticipant, NewCombatSession},
    sounds::NewSound,
    Database, DatabaseError,
};

pub fn open_database() -> Result<Database, DatabaseError> {
    Database::open(":memory:")
}

pub fn create_campaign(database: &Database) -> Result<Campaign, DatabaseError> {
    database.add_campaign(NewCampaign {
        name: "Test Campaign".to_string(),
        notes: None,
    })
}

pub fn new_character(
    campaign_id: i64,
    name: &str,
    kind: &str,
    current_health: i64,
    max_health: i64,
    armor_class: i64,
    notes: &str,
) -> NewCharacter {
    NewCharacter {
        campaign_id,
        name: name.to_string(),
        kind: kind.to_string(),
        current_health,
        max_health,
        armor_class,
        notes: notes.to_string(),
    }
}

pub fn new_combat_session(campaign_id: i64) -> NewCombatSession {
    NewCombatSession {
        campaign_id,
        name: "Goblin Ambush".to_string(),
        status: "planned".to_string(),
        current_round: 0,
        notes: "Roadside encounter".to_string(),
        details: json!({"ruleset": "dnd5e"}),
    }
}

pub fn new_combat_participant(
    combat_session_id: i64,
    character_id: Option<i64>,
) -> NewCombatParticipant {
    NewCombatParticipant {
        combat_session_id,
        character_id,
        display_name: "Goblin 1".to_string(),
        initiative: Some(14),
        turn_order: 1,
        current_health: Some(7),
        temporary_health: 0,
        defeated: false,
        details: json!({"source": "template"}),
    }
}

pub fn new_sound(title: &str, file_path: &str, category: &str, volume: f64) -> NewSound {
    NewSound {
        title: title.to_string(),
        file_path: file_path.to_string(),
        category: category.to_string(),
        volume,
    }
}
