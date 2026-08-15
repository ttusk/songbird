use crate::services::database::{Database, DatabaseError};
use rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Character {
    pub id: i64,
    pub campaign_id: i64,
    pub name: String,
    pub kind: String,
    pub current_health: i64,
    pub max_health: i64,
    pub armor_class: i64,
    pub notes: String,
}
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewCharacter {
    pub campaign_id: i64,
    pub name: String,
    pub kind: String,
    pub current_health: i64,
    pub max_health: i64,
    pub armor_class: i64,
    pub notes: String,
}
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdateCharacter {
    pub name: String,
    pub kind: String,
    pub current_health: i64,
    pub max_health: i64,
    pub armor_class: i64,
    pub notes: String,
}

impl Character {
    fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            campaign_id: row.get("campaign_id")?,
            name: row.get("name")?,
            kind: row.get("kind")?,
            current_health: row.get("current_hp")?,
            max_health: row.get("max_hp")?,
            armor_class: row.get("armor_class")?,
            notes: row.get("notes")?,
        })
    }
}

impl Database {
    pub fn list_characters(&self, id: i64) -> Result<Vec<Character>, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;

        let mut statement = connection.prepare(
            "SELECT id, campaign_id, name, kind, current_hp, max_hp, armor_class, notes
            FROM characters
            WHERE campaign_id = ?1",
        )?;

        let characters = statement
            .query_map([id], Character::from_row)?
            .collect::<rusqlite::Result<Vec<Character>>>()?;

        Ok(characters)
    }

    pub fn add_character(&self, new_character: NewCharacter) -> Result<Character, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;

        let NewCharacter {
            campaign_id,
            name,
            kind,
            current_health,
            max_health,
            armor_class,
            notes,
        } = new_character;

        connection.execute(
            "
            INSERT INTO characters (
                campaign_id,
                name,
                kind,
                current_hp,
                max_hp,
                armor_class,
                notes
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            ",
            rusqlite::params![
                campaign_id,
                &name,
                &kind,
                current_health,
                max_health,
                armor_class,
                &notes,
            ],
        )?;

        let id = connection.last_insert_rowid();

        Ok(Character {
            id,
            campaign_id,
            name,
            kind,
            current_health,
            max_health,
            armor_class,
            notes,
        })
    }

    pub fn find_character(&self, id: i64) -> Result<Option<Character>, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;

        let character = connection
            .query_row(
                "
                SELECT id, campaign_id, name, kind, current_hp, max_hp, armor_class, notes
                FROM characters
                WHERE id = ?1
                ",
                [id],
                Character::from_row,
            )
            .optional()?;

        Ok(character)
    }

    pub fn update_character(
        &self,
        id: i64,
        character: UpdateCharacter,
    ) -> Result<Option<Character>, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;

        let updated = connection.execute(
            "
            UPDATE characters
            SET name = ?1,
                kind = ?2,
                current_hp = ?3,
                max_hp = ?4,
                armor_class = ?5,
                notes = ?6
            WHERE id = ?7
            ",
            rusqlite::params![
                &character.name,
                &character.kind,
                character.current_health,
                character.max_health,
                character.armor_class,
                &character.notes,
                id,
            ],
        )?;

        drop(connection);

        if updated == 0 {
            return Ok(None);
        }

        self.find_character(id)
    }

    pub fn delete_character(&self, id: i64) -> Result<bool, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;
        let deleted = connection.execute("DELETE FROM characters WHERE id = ?1", [id])?;

        Ok(deleted == 1)
    }
}
