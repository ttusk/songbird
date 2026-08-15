use crate::services::database::{Database, DatabaseError};

#[derive(Debug, PartialEq, Eq)]
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
#[derive(Debug, PartialEq, Eq)]
pub struct NewCharacter {
    pub campaign_id: i64,
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
    pub(crate) fn list_characters(&self, id: i64) -> Result<Vec<Character>, DatabaseError> {
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

    pub(crate) fn add_character(
        &self,
        new_character: NewCharacter,
    ) -> Result<Character, DatabaseError> {
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
}

#[cfg(test)]
mod tests {
    use super::{Character, NewCharacter};
    use crate::services::database::{campaign::NewCampaign, Database, DatabaseError};

    #[test]
    fn add_new_character_returns_character() -> Result<(), DatabaseError> {
        let db = Database::open(":memory:")?;
        let campaign = db.add_campaign(NewCampaign {
            name: "Test Campaign".to_string(),
            notes: None,
        })?;

        let result = db.add_character(NewCharacter {
            campaign_id: campaign.id,
            name: "Goblin".to_string(),
            kind: "npc".to_string(),
            current_health: 7,
            max_health: 7,
            armor_class: 15,
            notes: "Guards the entrance".to_string(),
        })?;

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
        let db = Database::open(":memory:")?;
        let campaign = db.add_campaign(NewCampaign {
            name: "Test Campaign".to_string(),
            notes: None,
        })?;

        let first = NewCharacter {
            campaign_id: campaign.id,
            name: "Goblin".to_string(),
            kind: "npc".to_string(),
            current_health: 7,
            max_health: 7,
            armor_class: 15,
            notes: "Guards the entrance".to_string(),
        };

        let second = NewCharacter {
            campaign_id: campaign.id,
            name: "Archer".to_string(),
            kind: "npc".to_string(),
            current_health: 10,
            max_health: 10,
            armor_class: 13,
            notes: "Keeps distance".to_string(),
        };

        let expected = vec![db.add_character(first)?, db.add_character(second)?];

        let result = db.list_characters(campaign.id)?;

        assert_eq!(expected, result);

        Ok(())
    }
}
