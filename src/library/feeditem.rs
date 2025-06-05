use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FeedItem {
    pub title: String,
    pub description: String,
    pub url: String,
    pub author: String,
    pub slug: String,
}
