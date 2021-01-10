//! This module describes survey model

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
#[serde(tag = "kind")]
pub enum Question {
    Email { name: String, description: String },
    Text { name: String, description: String },
    Number { name: String, description: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewSurvey {
    name: String,
    description: String,
    questions: Vec<Question>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilledSurvey {
    survey_name: String,
    responses: HashMap<Question, String>,
}
