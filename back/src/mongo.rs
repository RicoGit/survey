//! Mongo utils module

use crate::survey::Survey;
use anyhow::{Context, Result};

use futures::stream::StreamExt;
use log::info;
use mongodb::bson::{doc, Document};
use mongodb::{bson, Database};
use serde::de::DeserializeOwned;

use std::any::TypeId;
use wither::mongodb::Client;

/// Connects db and sends check request
pub async fn init() -> Result<Database> {
    let db = Client::with_uri_str("mongodb://localhost:27017/")
        .await?
        .database("survey");

    // Ping the server to see if you can connect to the cluster
    db.run_command(doc! {"ping": 1}, None).await?;

    info!("MongoDb was connected successfully.");

    Ok(db)
}

/// Retrieves all surveys
pub async fn get_survey_list(db: &Client) -> Result<Vec<Survey>> {
    let surveys: Vec<Result<Survey>> = db
        .database("survey")
        .collection("surveys")
        .find(None, None)
        .await?
        .map(|res| res.map(as_struct).context("Deserialize fail"))
        .collect()
        .await;

    println!("{:?}", surveys);
    surveys.into_iter().collect::<Result<Vec<Survey>>>()
}

/// Converts Document to type T
fn as_struct<T: DeserializeOwned + Default + 'static>(doc: Document) -> T {
    println!("{:?}", doc);
    bson::from_bson::<T>(bson::Bson::Document(doc)).expect(&format!(
        "Deserialization fail for struct: {:?}",
        TypeId::of::<T>()
    ))
}
