mod common;

use common::{create_campaign, new_character, open_database};
use songbird_lib::services::database::{characters::Character, DatabaseError};

#[test]
fn add_new_character_returns_character() -> Result<(), DatabaseError> {
    let database = open_database()?;
    let campaign = create_campaign(&database)?;

    let result = database.add_character(new_character(
        campaign.id,
        "Goblin",
        "npc",
        7,
        7,
        15,
        "Guards the entrance",
    ))?;

    let expected = Character {
        id: 1,
        campaign_id: campaign.id,
        name: "Goblin".to_string(),
        kind: "npc".to_string(),
        current_health: 7,
        max_health: 7,
        armor_class: 15,
        notes: "Guards the entrance".to_string(),
    };

    assert_eq!(expected, result);

    Ok(())
}

#[test]
fn list_characters_from_campaign_returns_characters() -> Result<(), DatabaseError> {
    let database = open_database()?;
    let campaign = create_campaign(&database)?;
    let first = database.add_character(new_character(
        campaign.id,
        "Goblin",
        "npc",
        7,
        7,
        15,
        "Guards the entrance",
    ))?;
    let second = database.add_character(new_character(
        campaign.id,
        "Archer",
        "npc",
        10,
        10,
        13,
        "Keeps distance",
    ))?;

    let result = database.list_characters(campaign.id)?;

    assert_eq!(vec![first, second], result);

    Ok(())
}
