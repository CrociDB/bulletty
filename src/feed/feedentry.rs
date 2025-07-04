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
}
