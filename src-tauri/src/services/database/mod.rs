pub mod campaign;
pub mod characters;
pub mod sounds;

use rusqlite::Connection;
use std::sync::Mutex;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("mutex was poisoned")]
    Lock,
}

pub struct Database {
    connection: Mutex<Connection>,
}

impl Database {
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, DatabaseError> {
        let database = Self {
            connection: Mutex::new(Connection::open(path)?),
        };

        database.create_tables()?;

        Ok(database)
    }

    fn create_tables(&self) -> Result<(), DatabaseError> {
        let db = self.connection.lock().map_err(|_| DatabaseError::Lock)?;

        db.execute_batch(
            "
            PRAGMA foreign_keys = ON;

            CREATE TABLE IF NOT EXISTS campaigns (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                notes TEXT NOT NULL DEFAULT ''
            );

            CREATE TABLE IF NOT EXISTS characters (
                id INTEGER PRIMARY KEY,
                campaign_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                kind TEXT NOT NULL DEFAULT 'npc',
                current_hp INTEGER,
                max_hp INTEGER,
                armor_class INTEGER,
                notes TEXT NOT NULL DEFAULT '',
                FOREIGN KEY (campaign_id) REFERENCES campaigns(id)
            );

            CREATE TABLE IF NOT EXISTS character_conditions (
                character_id INTEGER NOT NULL,
                condition TEXT NOT NULL,
                PRIMARY KEY (character_id, condition),
                FOREIGN KEY (character_id) REFERENCES characters(id)
            );

            CREATE TABLE IF NOT EXISTS sounds (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                file_path TEXT NOT NULL,
                category TEXT NOT NULL DEFAULT '',
                volume REAL NOT NULL DEFAULT 1.0
            );
            ",
        )?;

        Ok(())
    }
}
