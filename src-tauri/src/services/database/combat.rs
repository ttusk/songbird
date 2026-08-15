use crate::services::database::{Database, DatabaseError};
use rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};
use serde_json::Value;

fn details_from_row(row: &rusqlite::Row<'_>, index: usize) -> rusqlite::Result<Value> {
    let details: String = row.get(index)?;

    serde_json::from_str(&details).map_err(|error| {
        rusqlite::Error::FromSqlConversionFailure(
            index,
            rusqlite::types::Type::Text,
            Box::new(error),
        )
    })
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CombatSession {
    pub id: i64,
    pub campaign_id: i64,
    pub name: String,
    pub status: String,
    pub current_round: i64,
    pub notes: String,
    pub details: Value,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NewCombatSession {
    pub campaign_id: i64,
    pub name: String,
    pub status: String,
    pub current_round: i64,
    pub notes: String,
    pub details: Value,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateCombatSession {
    pub name: String,
    pub status: String,
    pub current_round: i64,
    pub notes: String,
    pub details: Value,
}

impl CombatSession {
    fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            campaign_id: row.get("campaign_id")?,
            name: row.get("name")?,
            status: row.get("status")?,
            current_round: row.get("current_round")?,
            notes: row.get("notes")?,
            details: details_from_row(row, 6)?,
        })
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CombatParticipant {
    pub id: i64,
    pub combat_session_id: i64,
    pub character_id: Option<i64>,
    pub display_name: String,
    pub initiative: Option<i64>,
    pub turn_order: i64,
    pub current_health: Option<i64>,
    pub temporary_health: i64,
    pub defeated: bool,
    pub details: Value,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NewCombatParticipant {
    pub combat_session_id: i64,
    pub character_id: Option<i64>,
    pub display_name: String,
    pub initiative: Option<i64>,
    pub turn_order: i64,
    pub current_health: Option<i64>,
    pub temporary_health: i64,
    pub defeated: bool,
    pub details: Value,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateCombatParticipant {
    pub display_name: String,
    pub initiative: Option<i64>,
    pub turn_order: i64,
    pub current_health: Option<i64>,
    pub temporary_health: i64,
    pub defeated: bool,
    pub details: Value,
}

impl CombatParticipant {
    fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            combat_session_id: row.get("combat_session_id")?,
            character_id: row.get("character_id")?,
            display_name: row.get("display_name")?,
            initiative: row.get("initiative")?,
            turn_order: row.get("turn_order")?,
            current_health: row.get("current_health")?,
            temporary_health: row.get("temporary_health")?,
            defeated: row.get("defeated")?,
            details: details_from_row(row, 9)?,
        })
    }
}

impl Database {
    pub fn add_combat_session(
        &self,
        new_session: NewCombatSession,
    ) -> Result<CombatSession, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;
        let details = serde_json::to_string(&new_session.details)?;

        connection.execute(
            "
            INSERT INTO combat_sessions (
                campaign_id, name, status, current_round, notes, details
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            ",
            rusqlite::params![
                new_session.campaign_id,
                &new_session.name,
                &new_session.status,
                new_session.current_round,
                &new_session.notes,
                details,
            ],
        )?;

        let id = connection.last_insert_rowid();

        Ok(CombatSession {
            id,
            campaign_id: new_session.campaign_id,
            name: new_session.name,
            status: new_session.status,
            current_round: new_session.current_round,
            notes: new_session.notes,
            details: new_session.details,
        })
    }

    pub fn find_combat_session(&self, id: i64) -> Result<Option<CombatSession>, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;

        let session = connection
            .query_row(
                "
                SELECT id, campaign_id, name, status, current_round, notes, details
                FROM combat_sessions
                WHERE id = ?1
                ",
                [id],
                CombatSession::from_row,
            )
            .optional()?;

        Ok(session)
    }

    pub fn list_combat_sessions(
        &self,
        campaign_id: i64,
    ) -> Result<Vec<CombatSession>, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;

        let mut statement = connection.prepare(
            "
            SELECT id, campaign_id, name, status, current_round, notes, details
            FROM combat_sessions
            WHERE campaign_id = ?1
            ORDER BY id
            ",
        )?;

        let sessions = statement
            .query_map([campaign_id], CombatSession::from_row)?
            .collect::<rusqlite::Result<Vec<CombatSession>>>()?;

        Ok(sessions)
    }

    pub fn update_combat_session(
        &self,
        id: i64,
        session: UpdateCombatSession,
    ) -> Result<Option<CombatSession>, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;
        let details = serde_json::to_string(&session.details)?;

        let updated = connection.execute(
            "
            UPDATE combat_sessions
            SET name = ?1, status = ?2, current_round = ?3, notes = ?4, details = ?5
            WHERE id = ?6
            ",
            rusqlite::params![
                &session.name,
                &session.status,
                session.current_round,
                &session.notes,
                details,
                id,
            ],
        )?;

        drop(connection);

        if updated == 0 {
            return Ok(None);
        }

        self.find_combat_session(id)
    }

    pub fn delete_combat_session(&self, id: i64) -> Result<bool, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;
        let deleted = connection.execute("DELETE FROM combat_sessions WHERE id = ?1", [id])?;

        Ok(deleted == 1)
    }

    pub fn add_combat_participant(
        &self,
        new_participant: NewCombatParticipant,
    ) -> Result<CombatParticipant, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;
        let details = serde_json::to_string(&new_participant.details)?;

        connection.execute(
            "
            INSERT INTO combat_participants (
                combat_session_id,
                character_id,
                display_name,
                initiative,
                turn_order,
                current_health,
                temporary_health,
                defeated,
                details
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
            ",
            rusqlite::params![
                new_participant.combat_session_id,
                new_participant.character_id,
                &new_participant.display_name,
                new_participant.initiative,
                new_participant.turn_order,
                new_participant.current_health,
                new_participant.temporary_health,
                new_participant.defeated,
                details,
            ],
        )?;

        let id = connection.last_insert_rowid();

        Ok(CombatParticipant {
            id,
            combat_session_id: new_participant.combat_session_id,
            character_id: new_participant.character_id,
            display_name: new_participant.display_name,
            initiative: new_participant.initiative,
            turn_order: new_participant.turn_order,
            current_health: new_participant.current_health,
            temporary_health: new_participant.temporary_health,
            defeated: new_participant.defeated,
            details: new_participant.details,
        })
    }

    pub fn find_combat_participant(
        &self,
        id: i64,
    ) -> Result<Option<CombatParticipant>, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;

        let participant = connection
            .query_row(
                "
                SELECT id, combat_session_id, character_id, display_name,
                       initiative, turn_order, current_health, temporary_health,
                       defeated, details
                FROM combat_participants
                WHERE id = ?1
                ",
                [id],
                CombatParticipant::from_row,
            )
            .optional()?;

        Ok(participant)
    }

    pub fn list_combat_participants(
        &self,
        combat_session_id: i64,
    ) -> Result<Vec<CombatParticipant>, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;

        let mut statement = connection.prepare(
            "
            SELECT id, combat_session_id, character_id, display_name,
                   initiative, turn_order, current_health, temporary_health,
                   defeated, details
            FROM combat_participants
            WHERE combat_session_id = ?1
            ORDER BY turn_order, id
            ",
        )?;

        let participants = statement
            .query_map([combat_session_id], CombatParticipant::from_row)?
            .collect::<rusqlite::Result<Vec<CombatParticipant>>>()?;

        Ok(participants)
    }

    pub fn update_combat_participant(
        &self,
        id: i64,
        participant: UpdateCombatParticipant,
    ) -> Result<Option<CombatParticipant>, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;
        let details = serde_json::to_string(&participant.details)?;

        let updated = connection.execute(
            "
            UPDATE combat_participants
            SET display_name = ?1,
                initiative = ?2,
                turn_order = ?3,
                current_health = ?4,
                temporary_health = ?5,
                defeated = ?6,
                details = ?7
            WHERE id = ?8
            ",
            rusqlite::params![
                &participant.display_name,
                participant.initiative,
                participant.turn_order,
                participant.current_health,
                participant.temporary_health,
                participant.defeated,
                details,
                id,
            ],
        )?;

        drop(connection);

        if updated == 0 {
            return Ok(None);
        }

        self.find_combat_participant(id)
    }

    pub fn delete_combat_participant(&self, id: i64) -> Result<bool, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;
        let deleted = connection.execute("DELETE FROM combat_participants WHERE id = ?1", [id])?;

        Ok(deleted == 1)
    }
}
