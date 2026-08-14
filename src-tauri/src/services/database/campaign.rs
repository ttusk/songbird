use super::characters::Characters;
use crate::services::database::{Database, DatabaseError};
use rusqlite::OptionalExtension;

#[derive(Debug, PartialEq, Eq)]
pub struct Campaign {
    pub id: i64,
    pub name: String,
    pub notes: String,
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

pub struct CampaignDetails {
    pub campaign: Campaign,
    pub characters: Vec<Characters>,
}

impl Database {
    pub fn add_campaign(
        &self,
        name: String,
        notes: Option<String>,
    ) -> Result<Campaign, DatabaseError> {
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
}

#[cfg(test)]
mod tests {
    use crate::services::database::{campaign::Campaign, Database, DatabaseError};

    #[test]
    fn add_campaign_return_campaign() -> Result<(), DatabaseError> {
        let expected_res = Campaign {
            id: 1,
            name: "Test".to_string(),
            notes: "".to_string(),
        };

        let db = Database::open(":memory:")?;

        let res = db.add_campaign("Test".to_string(), None)?;

        assert_eq!(expected_res, res);

        Ok(())
    }
}
