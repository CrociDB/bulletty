use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::core::feed::feedentry::FeedEntry;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReadLaterEntry {
    pub path: Option<String>,
    pub date_added: DateTime<Utc>,
}

impl ReadLaterEntry {
    pub fn new(file_path: String) -> Self {
        Self {
            path: Some(file_path),
            date_added: Utc::now(),
        }
    }

    pub fn from_feed_entry(entry: &FeedEntry) -> Self {
        // Convert absolute path -> relative path
        let path = if let Some(path) = entry.filepath.to_str() {
            if let Some(categories_pos) = path.find("categories/") {
                let relative_path = &path[categories_pos + "categories/".len()..];
                relative_path.to_string()
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        };

        Self {
            path: Some(path),
            date_added: Utc::now(),
        }
    }

    pub fn to_feed_entry(&self) -> FeedEntry {
        FeedEntry {
            title: "".to_string(),
            description: "".to_string(),
            date: self.date_added,
            url: "".to_string(),
            author: "".to_string(),
            text: "".to_string(),
            lastupdated: self.date_added,
            seen: false,
            filepath: PathBuf::new(),
        }
    }
}
