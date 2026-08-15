use super::characters::Character;
use crate::services::database::{Database, DatabaseError};
use rusqlite::OptionalExtension;

#[derive(Debug, PartialEq, Eq)]
pub struct Campaign {
    pub id: i64,
    pub name: String,
    pub notes: String,
}
#[derive(Debug, PartialEq, Eq)]
pub struct NewCampaign {
    pub name: String,
    pub notes: Option<String>,
}

impl Campaign {
    fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            name: row.get("name")?,
            notes: row.get("notes")?,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CampaignDetails {
    pub campaign: Campaign,
    pub characters: Vec<Character>,
}

impl Database {
    pub fn add_campaign(&self, new_campaign: NewCampaign) -> Result<Campaign, DatabaseError> {
        let NewCampaign { name, notes } = new_campaign;
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;

        let notes = match notes {
            Some(value) => value,
            None => String::new(),
        };

        connection.execute(
            "
            INSERT INTO campaigns (name, notes) VALUES (?1, ?2)",
            (&name, &notes),
        )?;

        let id = connection.last_insert_rowid();

        Ok(Campaign { id, name, notes })
    }

    pub fn find_campaign(&self, id: i64) -> Result<Option<Campaign>, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;

        let campaign = connection
            .query_row(
                "SELECT id, name, notes FROM campaigns WHERE id = ?1",
                [id],
                Campaign::from_row,
            )
            .optional()?;

        Ok(campaign)
    }

    pub fn list_campaigns(&self) -> Result<Vec<Campaign>, DatabaseError> {
        let connection = self.connection.lock().map_err(|_| DatabaseError::Lock)?;

        let mut statement =
            connection.prepare("SELECT id, name, notes FROM campaigns ORDER BY name")?;

        let campaigns = statement
            .query_map([], Campaign::from_row)?
            .collect::<rusqlite::Result<Vec<Campaign>>>()?;

        Ok(campaigns)
    }

    pub fn find_campaign_details(&self, id: i64) -> Result<Option<CampaignDetails>, DatabaseError> {
        let Some(campaign) = self.find_campaign(id)? else {
            return Ok(None);
        };

        let characters = self.list_characters(id)?;

        Ok(Some(CampaignDetails {
            campaign,
            characters,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::{CampaignDetails, Character, NewCampaign};
    use crate::services::database::{Database, DatabaseError};

    #[test]
    fn add_campaign_returns_campaign() -> Result<(), DatabaseError> {
        let expected = super::Campaign {
            id: 1,
            name: "Test".to_string(),
            notes: String::new(),
        };

        let db = Database::open(":memory:")?;
        let result = db.add_campaign(NewCampaign {
            name: "Test".to_string(),
            notes: None,
        })?;

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn find_campaign_details_returns_campaign_details() -> Result<(), DatabaseError> {
        let db = Database::open(":memory:")?;
        let campaign = db.add_campaign(NewCampaign {
            name: "Test Campaign".to_string(),
            notes: Some("Test notes".to_string()),
        })?;

        let expected_characters = vec![
            Character {
                id: 1,
                campaign_id: campaign.id,
                name: "Goblin".to_string(),
                kind: "npc".to_string(),
                current_health: 7,
                max_health: 7,
                armor_class: 15,
                notes: "Guards the entrance".to_string(),
            },
            Character {
                id: 2,
                campaign_id: campaign.id,
                name: "Archer".to_string(),
                kind: "npc".to_string(),
                current_health: 10,
                max_health: 10,
                armor_class: 13,
                notes: "Keeps distance".to_string(),
            },
        ];

        {
            let connection = db.connection.lock().map_err(|_| DatabaseError::Lock)?;

            for character in &expected_characters {
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
                    (
                        campaign.id,
                        &character.name,
                        &character.kind,
                        character.current_health,
                        character.max_health,
                        character.armor_class,
                        &character.notes,
                    ),
                )?;
            }
        }

        let expected = CampaignDetails {
            campaign,
            characters: expected_characters,
        };

        let result = db.find_campaign_details(expected.campaign.id)?;

        assert_eq!(Some(expected), result);

        Ok(())
    }

    #[test]
    fn find_campaign_returns_campaign() -> Result<(), DatabaseError> {
        let db = Database::open(":memory:")?;
        let expected = db.add_campaign(NewCampaign {
            name: "Test Campaign".to_string(),
            notes: Some("Test notes".to_string()),
        })?;
        let id = expected.id;

        let result = db.find_campaign(id)?;

        assert_eq!(Some(expected), result);

        Ok(())
    }

    #[test]
    fn find_campaign_returns_none_for_missing_id() -> Result<(), DatabaseError> {
        let db = Database::open(":memory:")?;

        let result = db.find_campaign(999)?;

        assert_eq!(None, result);

        Ok(())
    }

    #[test]
    fn list_campaigns_returns_campaigns_by_name() -> Result<(), DatabaseError> {
        let db = Database::open(":memory:")?;
        let second = db.add_campaign(NewCampaign {
            name: "B Campaign".to_string(),
            notes: None,
        })?;
        let first = db.add_campaign(NewCampaign {
            name: "A Campaign".to_string(),
            notes: None,
        })?;

        let result = db.list_campaigns()?;

        assert_eq!(vec![first, second], result);

        Ok(())
    }

    #[test]
    fn find_campaign_details_returns_none_for_missing_id() -> Result<(), DatabaseError> {
        let db = Database::open(":memory:")?;

        let result = db.find_campaign_details(999)?;

        assert_eq!(None, result);

        Ok(())
    }
}
