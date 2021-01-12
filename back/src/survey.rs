//! This module describes survey model

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use wither::bson::doc;
use wither::bson::oid::ObjectId;
use wither::prelude::*;

#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
#[serde(tag = "kind")]
pub enum Question {
    Email { name: String, description: String },
    Text { name: String, description: String },
    Number { name: String, description: String },
}

#[derive(Model, Serialize, Deserialize, Debug, Default)]
#[model()]
pub struct Survey {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    description: String,
    questions: Vec<Question>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilledSurvey {
    survey_name: String,
    responses: HashMap<Question, String>,
}
