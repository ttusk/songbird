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

#[derive(Debug, PartialEq, Eq)]
pub struct CampaignDetails {
    pub campaign: Campaign,
    pub characters: Vec<Character>,
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
