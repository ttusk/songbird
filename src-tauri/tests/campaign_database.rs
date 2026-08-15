mod common;

use common::{create_campaign, new_character, open_database};
use songbird_lib::services::database::{
    campaign::{Campaign, CampaignDetails, NewCampaign, UpdateCampaign},
    DatabaseError,
};

#[test]
fn add_campaign_returns_campaign() -> Result<(), DatabaseError> {
    let database = open_database()?;
    let expected = Campaign {
        id: 1,
        name: "Test".to_string(),
        notes: String::new(),
    };

    let result = database.add_campaign(NewCampaign {
        name: "Test".to_string(),
        notes: None,
    })?;

    assert_eq!(expected, result);

    Ok(())
}

#[test]
fn find_campaign_returns_campaign() -> Result<(), DatabaseError> {
    let database = open_database()?;
    let expected = create_campaign(&database)?;
    let id = expected.id;

    let result = database.find_campaign(id)?;

    assert_eq!(Some(expected), result);

    Ok(())
}

#[test]
fn find_campaign_returns_none_for_missing_id() -> Result<(), DatabaseError> {
    let database = open_database()?;

    let result = database.find_campaign(999)?;

    assert_eq!(None, result);

    Ok(())
}

#[test]
fn list_campaigns_returns_campaigns_by_name() -> Result<(), DatabaseError> {
    let database = open_database()?;
    let second = database.add_campaign(NewCampaign {
        name: "B Campaign".to_string(),
        notes: None,
    })?;
    let first = database.add_campaign(NewCampaign {
        name: "A Campaign".to_string(),
        notes: None,
    })?;

    let result = database.list_campaigns()?;

    assert_eq!(vec![first, second], result);

    Ok(())
}

#[test]
fn find_campaign_details_returns_campaign_details() -> Result<(), DatabaseError> {
    let database = open_database()?;
    let campaign = create_campaign(&database)?;
    let first = database.add_character(new_character(
        campaign.id,
        "Goblin",
        "npc",
        7,
        7,
        15,
        "Guards the entrance",
    ))?;
    let second = database.add_character(new_character(
        campaign.id,
        "Archer",
        "npc",
        10,
        10,
        13,
        "Keeps distance",
    ))?;

    let expected = CampaignDetails {
        campaign,
        characters: vec![first, second],
    };
    let result = database.find_campaign_details(expected.campaign.id)?;

    assert_eq!(Some(expected), result);

    Ok(())
}

#[test]
fn find_campaign_details_returns_none_for_missing_id() -> Result<(), DatabaseError> {
    let database = open_database()?;

    let result = database.find_campaign_details(999)?;

    assert_eq!(None, result);

    Ok(())
}

#[test]
fn update_campaign_returns_updated_campaign() -> Result<(), DatabaseError> {
    let database = open_database()?;
    let campaign = create_campaign(&database)?;

    let result = database.update_campaign(
        campaign.id,
        UpdateCampaign {
            name: "Updated Campaign".to_string(),
            notes: "Updated notes".to_string(),
        },
    )?;

    assert_eq!(
        Some(Campaign {
            id: campaign.id,
            name: "Updated Campaign".to_string(),
            notes: "Updated notes".to_string(),
        }),
        result,
    );

    Ok(())
}

#[test]
fn update_campaign_returns_none_for_missing_id() -> Result<(), DatabaseError> {
    let database = open_database()?;

    let result = database.update_campaign(
        999,
        UpdateCampaign {
            name: "Updated Campaign".to_string(),
            notes: "Updated notes".to_string(),
        },
    )?;

    assert_eq!(None, result);

    Ok(())
}

#[test]
fn delete_campaign_removes_campaign() -> Result<(), DatabaseError> {
    let database = open_database()?;
    let campaign = create_campaign(&database)?;

    assert!(database.delete_campaign(campaign.id)?);
    assert_eq!(None, database.find_campaign(campaign.id)?);
    assert!(!database.delete_campaign(campaign.id)?);

    Ok(())
}
