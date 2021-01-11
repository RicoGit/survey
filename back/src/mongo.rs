//! Mongo utils module

use crate::survey::NewSurvey;
use anyhow::Result;
use log::info;
use mongodb::{bson, bson::doc, options::ClientOptions, Client};

pub async fn init() -> Result<Client> {
    // Parse your connection string into an options struct
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    // Manually set an option
    client_options.app_name = Some("Survey app".to_string());

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    // Ping the server to see if you can connect to the cluster
    client
        .database("survey")
        .run_command(doc! {"ping": 1}, None)
        .await?;

    info!("MongoDb was connected successfully.");

    Ok(client)
}

/// Saves new survey to mongoDb
pub async fn create_survey(db: &Client, survey: &NewSurvey) -> Result<()> {
    let doc = bson::to_bson(survey)?
        .as_document()
        .expect("NewSurvey should be serialized as bson document")
        .to_owned();

    db.database("survey")
        .collection("surveys")
        .insert_one(doc, None)
        .await?;
    Ok(())
}
