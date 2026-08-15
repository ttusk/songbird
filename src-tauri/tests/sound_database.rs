mod common;

use common::{new_sound, open_database};
use songbird_lib::services::database::{
    sounds::{Sound, UpdateSound},
    DatabaseError,
};

#[test]
fn add_sound_returns_sound() -> Result<(), DatabaseError> {
    let database = open_database()?;

    let result = database.add_sound(new_sound("Thunder", "sounds/thunder.mp3", "weather", 0.8))?;

    let expected = Sound {
        id: 1,
        title: "Thunder".to_string(),
        file_path: "sounds/thunder.mp3".to_string(),
        category: "weather".to_string(),
        volume: 0.8,
    };

    assert_eq!(expected, result);

    Ok(())
}

#[test]
fn find_sound_returns_sound() -> Result<(), DatabaseError> {
    let database = open_database()?;
    let expected = database.add_sound(new_sound("Door", "sounds/door.mp3", "environment", 1.0))?;
    let id = expected.id;

    let result = database.find_sound(id)?;

    assert_eq!(Some(expected), result);

    Ok(())
}

#[test]
fn find_sound_returns_none_for_missing_id() -> Result<(), DatabaseError> {
    let database = open_database()?;

    let result = database.find_sound(999)?;

    assert_eq!(None, result);

    Ok(())
}

#[test]
fn list_sounds_returns_sounds_by_title() -> Result<(), DatabaseError> {
    let database = open_database()?;
    let second = database.add_sound(new_sound("Thunder", "sounds/thunder.mp3", "weather", 0.8))?;
    let first = database.add_sound(new_sound("Door", "sounds/door.mp3", "environment", 1.0))?;

    let result = database.list_sounds()?;

    assert_eq!(vec![first, second], result);

    Ok(())
}

#[test]
fn delete_sound_removes_sound() -> Result<(), DatabaseError> {
    let database = open_database()?;
    let sound = database.add_sound(new_sound(
        "Explosion",
        "sounds/explosion.mp3",
        "combat",
        1.0,
    ))?;

    assert!(database.delete_sound(sound.id)?);
    assert_eq!(None, database.find_sound(sound.id)?);
    assert!(!database.delete_sound(sound.id)?);

    Ok(())
}

#[test]
fn update_sound_changes_metadata_and_volume() -> Result<(), DatabaseError> {
    let database = open_database()?;
    let sound = database.add_sound(new_sound("Old Name", "old.mp3", "environment", 0.5))?;

    let updated = database.update_sound(
        sound.id,
        UpdateSound {
            title: "New Name".to_string(),
            file_path: "new.mp3".to_string(),
            category: "combat".to_string(),
            volume: 0.75,
        },
    )?;

    assert_eq!(
        Some("New Name"),
        updated.as_ref().map(|item| item.title.as_str())
    );
    assert_eq!(Some(0.75), updated.as_ref().map(|item| item.volume));
    assert_eq!(
        None,
        database.update_sound(
            999,
            UpdateSound {
                title: "Missing".to_string(),
                file_path: "missing.mp3".to_string(),
                category: "none".to_string(),
                volume: 1.0,
            }
        )?
    );

    Ok(())
}
