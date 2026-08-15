pub mod campaign;
pub mod characters;
pub mod combat;
pub mod sounds;

use rusqlite::Connection;
use std::sync::Mutex;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
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
            CREATE TABLE IF NOT EXISTS combat_sessions (
                id INTEGER PRIMARY KEY,
                campaign_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'planned',
                current_round INTEGER NOT NULL DEFAULT 0,
                notes TEXT NOT NULL DEFAULT '',
                details TEXT NOT NULL DEFAULT '{}',
                FOREIGN KEY (campaign_id) REFERENCES campaigns(id)
            );

            CREATE TABLE IF NOT EXISTS combat_participants (
                id INTEGER PRIMARY KEY,
                combat_session_id INTEGER NOT NULL,
                character_id INTEGER,
                display_name TEXT NOT NULL,
                initiative INTEGER,
                turn_order INTEGER NOT NULL DEFAULT 0,
                current_health INTEGER,
                temporary_health INTEGER NOT NULL DEFAULT 0,
                defeated INTEGER NOT NULL DEFAULT 0,
                details TEXT NOT NULL DEFAULT '{}',
                FOREIGN KEY (combat_session_id) REFERENCES combat_sessions(id) ON DELETE CASCADE,
                FOREIGN KEY (character_id) REFERENCES characters(id)
            );
            ",
        )?;

        Ok(())
    }
}
