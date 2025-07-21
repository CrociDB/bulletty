use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct FeedEntry {
    pub title: String,
    pub description: String,
    pub date: DateTime<Utc>,
    pub url: String,
    pub author: String,
    pub text: String,

    pub lastupdated: DateTime<Utc>,
    pub seen: bool,

    #[serde(skip_serializing, skip_deserializing)]
    pub filepath: PathBuf,
}
