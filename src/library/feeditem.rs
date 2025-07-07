use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct FeedItem {
    pub title: String,
    pub description: String,
    pub url: String,
    pub author: String,
    pub slug: String,

    pub lastupdated: DateTime<Utc>,
}
