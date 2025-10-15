use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::core::feed::feedentry::FeedEntry;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReadLaterEntry {
    pub title: String,
    pub description: String,
    pub url: String,
    pub date_added: DateTime<Utc>,
    pub source_feed: Option<String>,
    pub source_category: Option<String>,
    pub file_path: Option<String>,
}

impl ReadLaterEntry {
    pub fn new(title: String, description: String, url: String) -> Self {
        Self {
            title,
            description,
            url,
            date_added: Utc::now(),
            source_feed: None,
            source_category: None,
            file_path: None,
        }
    }

    pub fn from_feed_entry(
        entry: &FeedEntry,
        source_feed: Option<String>,
        source_category: Option<String>,
    ) -> Self {
        // Extract file path
        let file_path = if let Some(path) = entry.filepath.to_str() {
            // Convert absolute path -> relative path
            if let Some(categories_pos) = path.find("categories/") {
                let relative_path = &path[categories_pos + "categories/".len()..];
                Some(relative_path.to_string())
            } else {
                None
            }
        } else {
            None
        };

        Self {
            title: entry.title.clone(),
            description: entry.description.clone(),
            url: entry.url.clone(),
            date_added: Utc::now(),
            source_feed,
            source_category,
            file_path,
        }
    }

    pub fn to_feed_entry(&self) -> FeedEntry {
        FeedEntry {
            title: self.title.clone(),
            description: self.description.clone(),
            date: self.date_added,
            url: self.url.clone(),
            author: "".to_string(),
            text: self.description.clone(),
            lastupdated: self.date_added,
            seen: false,
            filepath: PathBuf::new(),
        }
    }
}
