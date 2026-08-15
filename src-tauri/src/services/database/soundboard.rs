use crate::services::database::{Database, DatabaseError};
use rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Soundboard {
    pub id: i64,
    pub campaign_id: Option<i64>,
    pub name: String,
    pub notes: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NewSoundboard {
    pub campaign_id: Option<i64>,
    pub name: String,
    pub notes: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateSoundboard {
    pub name: String,
    pub notes: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SoundboardSound {
    pub soundboard_id: i64,
    pub sound_id: i64,
    pub position: i64,
    pub title: String,
    pub file_path: String,
    pub category: String,
    pub volume: f64,
}

impl Soundboard {
    fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            campaign_id: row.get("campaign_id")?,
            name: row.get("name")?,
            notes: row.get("notes")?,
        })
    }
}

impl SoundboardSound {
    fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            soundboard_id: row.get("soundboard_id")?,
            sound_id: row.get("sound_id")?,
            position: row.get("position")?,
            title: row.get("title")?,
            file_path: row.get("file_path")?,
            category: row.get("category")?,
            volume: row.get("volume")?,
        })
    }
}

const SOUNDBOARD_SOUND_SELECT: &str = "
    SELECT soundboard_sounds.soundboard_id,
           soundboard_sounds.sound_id,
           soundboard_sounds.position,
           sounds.title,
           sounds.file_path,
           sounds.category,
           sounds.volume
    FROM soundboard_sounds
    INNER JOIN sounds ON sounds.id = soundboard_sounds.sound_id
";

impl Database {
    pub fn add_soundboard(&self, soundboard: NewSoundboard) -> Result<Soundboard, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;

        connection.execute(
            "
            INSERT INTO soundboards (campaign_id, name, notes)
            VALUES (?1, ?2, ?3)
            ",
            rusqlite::params![soundboard.campaign_id, &soundboard.name, &soundboard.notes],
        )?;

        Ok(Soundboard {
            id: connection.last_insert_rowid(),
            campaign_id: soundboard.campaign_id,
            name: soundboard.name,
            notes: soundboard.notes,
        })
    }

    pub fn find_soundboard(&self, id: i64) -> Result<Option<Soundboard>, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;

        Ok(connection
            .query_row(
                "SELECT id, campaign_id, name, notes FROM soundboards WHERE id = ?1",
                [id],
                Soundboard::from_row,
            )
            .optional()?)
    }

    pub fn list_soundboards(&self) -> Result<Vec<Soundboard>, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;
        let mut statement = connection.prepare(
            "
            SELECT id, campaign_id, name, notes
            FROM soundboards
            ORDER BY name
            ",
        )?;

        let soundboards = statement
            .query_map([], Soundboard::from_row)?
            .collect::<rusqlite::Result<Vec<Soundboard>>>()?;

        Ok(soundboards)
    }

    pub fn update_soundboard(
        &self,
        id: i64,
        soundboard: UpdateSoundboard,
    ) -> Result<Option<Soundboard>, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;
        let updated = connection.execute(
            "UPDATE soundboards SET name = ?1, notes = ?2 WHERE id = ?3",
            rusqlite::params![&soundboard.name, &soundboard.notes, id],
        )?;

        drop(connection);

        if updated == 0 {
            return Ok(None);
        }

        self.find_soundboard(id)
    }

    pub fn delete_soundboard(&self, id: i64) -> Result<bool, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;
        Ok(connection.execute("DELETE FROM soundboards WHERE id = ?1", [id])? == 1)
    }

    pub fn add_sound_to_soundboard(
        &self,
        soundboard_id: i64,
        sound_id: i64,
        position: i64,
    ) -> Result<SoundboardSound, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;

        connection.execute(
            "
            INSERT INTO soundboard_sounds (soundboard_id, sound_id, position)
            VALUES (?1, ?2, ?3)
            ",
            rusqlite::params![soundboard_id, sound_id, position],
        )?;

        Ok(connection.query_row(
            &format!("{SOUNDBOARD_SOUND_SELECT} WHERE soundboard_sounds.soundboard_id = ?1 AND soundboard_sounds.sound_id = ?2"),
            [soundboard_id, sound_id],
            SoundboardSound::from_row,
        )?)
    }

    pub fn list_soundboard_sounds(
        &self,
        soundboard_id: i64,
    ) -> Result<Vec<SoundboardSound>, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;
        let mut statement = connection.prepare(&format!(
            "{SOUNDBOARD_SOUND_SELECT} WHERE soundboard_sounds.soundboard_id = ?1 ORDER BY soundboard_sounds.position"
        ))?;

        let sounds = statement
            .query_map([soundboard_id], SoundboardSound::from_row)?
            .collect::<rusqlite::Result<Vec<SoundboardSound>>>()?;

        Ok(sounds)
    }

    pub fn remove_sound_from_soundboard(
        &self,
        soundboard_id: i64,
        sound_id: i64,
    ) -> Result<bool, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;
        Ok(connection.execute(
            "DELETE FROM soundboard_sounds WHERE soundboard_id = ?1 AND sound_id = ?2",
            [soundboard_id, sound_id],
        )? == 1)
    }
}
