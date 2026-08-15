mod common;

use common::{create_campaign, new_sound, open_database};
use songbird_lib::services::database::{
    soundboard::{NewSoundboard, UpdateSoundboard},
    DatabaseError,
};

fn new_soundboard(campaign_id: Option<i64>, name: &str) -> NewSoundboard {
    NewSoundboard {
        campaign_id,
        name: name.to_string(),
        notes: "Atmosphere controls".to_string(),
    }
}

#[test]
fn add_find_list_update_and_delete_soundboards() -> Result<(), DatabaseError> {
    let database = open_database()?;
    let campaign = create_campaign(&database)?;
    let first = database.add_soundboard(new_soundboard(Some(campaign.id), "Weather"))?;
    let first_id = first.id;
    let second = database.add_soundboard(new_soundboard(None, "Combat"))?;

    assert_eq!(
        Some(first_id),
        database.find_soundboard(first_id)?.map(|board| board.id)
    );
    let first_found = database.find_soundboard(first_id)?.expect("first board");
    assert_eq!(vec![second, first_found], database.list_soundboards()?);

    let updated = database.update_soundboard(
        first_id,
        UpdateSoundboard {
            name: "Environment".to_string(),
            notes: "Rain and wind".to_string(),
        },
    )?;
    assert_eq!(
        Some("Environment"),
        updated.as_ref().map(|board| board.name.as_str())
    );

    assert!(database.delete_soundboard(first_id)?);
    assert_eq!(None, database.find_soundboard(first_id)?);

    Ok(())
}

#[test]
fn soundboard_sounds_are_ordered_and_removable() -> Result<(), DatabaseError> {
    let database = open_database()?;
    let board = database.add_soundboard(new_soundboard(None, "Ambience"))?;
    let thunder = database.add_sound(new_sound("Thunder", "thunder.mp3", "weather", 0.8))?;
    let door = database.add_sound(new_sound("Door", "door.mp3", "environment", 1.0))?;

    database.add_sound_to_soundboard(board.id, thunder.id, 1)?;
    database.add_sound_to_soundboard(board.id, door.id, 0)?;

    let sounds = database.list_soundboard_sounds(board.id)?;
    assert_eq!(
        vec![door.id, thunder.id],
        sounds
            .iter()
            .map(|sound| sound.sound_id)
            .collect::<Vec<_>>()
    );
    assert_eq!("Door", sounds[0].title);
    assert_eq!(0, sounds[0].position);

    assert!(database.remove_sound_from_soundboard(board.id, door.id)?);
    assert!(!database.remove_sound_from_soundboard(board.id, door.id)?);
    assert_eq!(
        thunder.id,
        database.list_soundboard_sounds(board.id)?[0].sound_id
    );

    Ok(())
}

#[test]
fn deleting_sound_removes_soundboard_entry() -> Result<(), DatabaseError> {
    let database = open_database()?;
    let board = database.add_soundboard(new_soundboard(None, "Effects"))?;
    let sound = database.add_sound(new_sound("Impact", "impact.wav", "combat", 1.0))?;
    database.add_sound_to_soundboard(board.id, sound.id, 0)?;

    assert!(database.delete_sound(sound.id)?);
    assert!(database.list_soundboard_sounds(board.id)?.is_empty());

    Ok(())
}
