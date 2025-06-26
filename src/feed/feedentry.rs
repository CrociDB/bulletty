use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct FeedEntry {
    pub title: String,
    pub description: String,
    pub date:DateTime<Utc>, 
    pub url: String,
    pub author: String,
    pub text: String,
}
