mod common;

use common::{create_campaign, new_character, open_database};
use songbird_lib::services::database::{
    characters::{Character, UpdateCharacter},
    DatabaseError,
};

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

#[test]
fn find_character_returns_character() -> Result<(), DatabaseError> {
    let database = open_database()?;
    let campaign = create_campaign(&database)?;
    let expected = database.add_character(new_character(
        campaign.id,
        "Goblin",
        "npc",
        7,
        7,
        15,
        "Guards the entrance",
    ))?;
    let id = expected.id;

    let result = database.find_character(id)?;

    assert_eq!(Some(expected), result);

    Ok(())
}

#[test]
fn find_character_returns_none_for_missing_id() -> Result<(), DatabaseError> {
    let database = open_database()?;

    let result = database.find_character(999)?;

    assert_eq!(None, result);

    Ok(())
}

#[test]
fn update_character_returns_updated_character() -> Result<(), DatabaseError> {
    let database = open_database()?;
    let campaign = create_campaign(&database)?;
    let character = database.add_character(new_character(
        campaign.id,
        "Goblin",
        "npc",
        7,
        7,
        15,
        "Guards the entrance",
    ))?;

    let result = database.update_character(
        character.id,
        UpdateCharacter {
            name: "Veteran Goblin".to_string(),
            kind: "elite".to_string(),
            current_health: 12,
            max_health: 12,
            armor_class: 17,
            notes: "Leads the guards".to_string(),
        },
    )?;

    assert_eq!(
        Some(Character {
            id: character.id,
            campaign_id: campaign.id,
            name: "Veteran Goblin".to_string(),
            kind: "elite".to_string(),
            current_health: 12,
            max_health: 12,
            armor_class: 17,
            notes: "Leads the guards".to_string(),
        }),
        result,
    );

    Ok(())
}

#[test]
fn update_character_returns_none_for_missing_id() -> Result<(), DatabaseError> {
    let database = open_database()?;

    let result = database.update_character(
        999,
        UpdateCharacter {
            name: "Missing".to_string(),
            kind: "npc".to_string(),
            current_health: 1,
            max_health: 1,
            armor_class: 10,
            notes: String::new(),
        },
    )?;

    assert_eq!(None, result);

    Ok(())
}

#[test]
fn delete_character_removes_character() -> Result<(), DatabaseError> {
    let database = open_database()?;
    let campaign = create_campaign(&database)?;
    let character = database.add_character(new_character(
        campaign.id,
        "Goblin",
        "npc",
        7,
        7,
        15,
        "Guards the entrance",
    ))?;

    assert!(database.delete_character(character.id)?);
    assert_eq!(None, database.find_character(character.id)?);
    assert!(!database.delete_character(character.id)?);

    Ok(())
}
