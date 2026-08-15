use songbird_lib::services::database::{
    campaign::{Campaign, NewCampaign},
    characters::NewCharacter,
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
