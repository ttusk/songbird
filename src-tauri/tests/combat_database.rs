mod common;

use common::{
    create_campaign, new_character, new_combat_participant, new_combat_session, open_database,
};
use serde_json::json;
use songbird_lib::services::database::{
    combat::{NewCombatSession, UpdateCombatParticipant, UpdateCombatSession},
    DatabaseError,
};

#[test]
fn add_and_find_combat_session() -> Result<(), DatabaseError> {
    let database = open_database()?;
    let campaign = create_campaign(&database)?;
    let expected = database.add_combat_session(new_combat_session(campaign.id))?;
    let id = expected.id;

    let result = database.find_combat_session(id)?;

    assert_eq!(Some(expected), result);

    Ok(())
}

#[test]
fn list_combat_sessions_filters_by_campaign() -> Result<(), DatabaseError> {
    let database = open_database()?;
    let first_campaign = create_campaign(&database)?;
    let second_campaign =
        database.add_campaign(songbird_lib::services::database::campaign::NewCampaign {
            name: "Second Campaign".to_string(),
            notes: None,
        })?;

    let first = database.add_combat_session(new_combat_session(first_campaign.id))?;
    database.add_combat_session(NewCombatSession {
        campaign_id: second_campaign.id,
        name: "Other Combat".to_string(),
        ..new_combat_session(second_campaign.id)
    })?;

    let result = database.list_combat_sessions(first_campaign.id)?;

    assert_eq!(vec![first], result);

    Ok(())
}

#[test]
fn update_and_delete_combat_session() -> Result<(), DatabaseError> {
    let database = open_database()?;
    let campaign = create_campaign(&database)?;
    let session = database.add_combat_session(new_combat_session(campaign.id))?;

    let updated = database.update_combat_session(
        session.id,
        UpdateCombatSession {
            name: "Updated Ambush".to_string(),
            status: "active".to_string(),
            current_round: 2,
            notes: "The fight has started".to_string(),
            details: json!({"ruleset": "dnd5e", "difficulty": "hard"}),
        },
    )?;

    assert_eq!(
        Some("Updated Ambush"),
        updated.as_ref().map(|item| item.name.as_str())
    );
    assert_eq!(Some(2), updated.as_ref().map(|item| item.current_round));

    assert!(database.delete_combat_session(session.id)?);
    assert_eq!(None, database.find_combat_session(session.id)?);

    Ok(())
}

#[test]
fn add_list_update_and_delete_combat_participant() -> Result<(), DatabaseError> {
    let database = open_database()?;
    let campaign = create_campaign(&database)?;
    let character = database.add_character(new_character(
        campaign.id,
        "Goblin",
        "enemy",
        7,
        7,
        15,
        "A reusable enemy template",
    ))?;
    let session = database.add_combat_session(new_combat_session(campaign.id))?;
    let participant =
        database.add_combat_participant(new_combat_participant(session.id, Some(character.id)))?;
    let participant_id = participant.id;

    assert_eq!(
        vec![participant],
        database.list_combat_participants(session.id)?
    );

    let updated = database.update_combat_participant(
        participant_id,
        UpdateCombatParticipant {
            display_name: "Goblin 1".to_string(),
            initiative: Some(18),
            turn_order: 1,
            current_health: Some(3),
            temporary_health: 2,
            defeated: false,
            details: json!({"condition": "wounded"}),
        },
    )?;

    assert_eq!(Some(18), updated.as_ref().and_then(|item| item.initiative));
    assert_eq!(
        Some(3),
        updated.as_ref().and_then(|item| item.current_health)
    );

    assert!(database.delete_combat_participant(participant_id)?);
    assert_eq!(None, database.find_combat_participant(participant_id)?);

    Ok(())
}

#[test]
fn deleting_combat_session_removes_participants() -> Result<(), DatabaseError> {
    let database = open_database()?;
    let campaign = create_campaign(&database)?;
    let session = database.add_combat_session(new_combat_session(campaign.id))?;
    let participant = database.add_combat_participant(new_combat_participant(session.id, None))?;

    assert!(database.delete_combat_session(session.id)?);
    assert_eq!(None, database.find_combat_participant(participant.id)?);

    Ok(())
}
