use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct FeedEntry {
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub date: String,
    pub url: String,
    pub author: String,
    pub text: String,
}
