use crate::services::database::{Database, DatabaseError};
use rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Sound {
    pub id: i64,
    pub title: String,
    pub file_path: String,
    pub category: String,
    pub volume: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NewSound {
    pub title: String,
    pub file_path: String,
    pub category: String,
    pub volume: f64,
}

impl Sound {
    fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            title: row.get("title")?,
            file_path: row.get("file_path")?,
            category: row.get("category")?,
            volume: row.get("volume")?,
        })
    }
}

impl Database {
    pub fn add_sound(&self, new_sound: NewSound) -> Result<Sound, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;

        let NewSound {
            title,
            file_path,
            category,
            volume,
        } = new_sound;

        connection.execute(
            "
            INSERT INTO sounds (title, file_path, category, volume)
            VALUES (?1, ?2, ?3, ?4)
            ",
            rusqlite::params![&title, &file_path, &category, volume],
        )?;

        let id = connection.last_insert_rowid();

        Ok(Sound {
            id,
            title,
            file_path,
            category,
            volume,
        })
    }

    pub fn find_sound(&self, id: i64) -> Result<Option<Sound>, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;

        let sound = connection
            .query_row(
                "SELECT id, title, file_path, category, volume FROM sounds WHERE id = ?1",
                [id],
                Sound::from_row,
            )
            .optional()?;

        Ok(sound)
    }

    pub fn list_sounds(&self) -> Result<Vec<Sound>, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;

        let mut statement = connection.prepare(
            "
            SELECT id, title, file_path, category, volume
            FROM sounds
            ORDER BY title
            ",
        )?;

        let sounds = statement
            .query_map([], Sound::from_row)?
            .collect::<rusqlite::Result<Vec<Sound>>>()?;

        Ok(sounds)
    }

    pub fn delete_sound(&self, id: i64) -> Result<bool, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;

        let deleted = connection.execute("DELETE FROM sounds WHERE id = ?1", [id])?;

        Ok(deleted == 1)
    }
}
