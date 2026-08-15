use crate::services::database::{
    campaign::{Campaign, CampaignDetails, NewCampaign},
    Database,
};
use tauri::State;

#[tauri::command]
pub fn add_campaign(
    database: State<'_, Database>,
    campaign: NewCampaign,
) -> Result<Campaign, String> {
    database
        .add_campaign(campaign)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn find_campaign(database: State<'_, Database>, id: i64) -> Result<Option<Campaign>, String> {
    database
        .find_campaign(id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn list_campaigns(database: State<'_, Database>) -> Result<Vec<Campaign>, String> {
    database.list_campaigns().map_err(|error| error.to_string())
}

#[tauri::command]
pub fn find_campaign_details(
    database: State<'_, Database>,
    id: i64,
) -> Result<Option<CampaignDetails>, String> {
    database
        .find_campaign_details(id)
        .map_err(|error| error.to_string())
}
