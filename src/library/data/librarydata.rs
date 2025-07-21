use std::io::Write;
use std::{
    fs::{self, OpenOptions},
    path::{Path, PathBuf},
};

use chrono::Utc;
use color_eyre::eyre::eyre;
use slug::slugify;
use tracing::{error, info};

use crate::feed::feedentry::FeedEntry;
use crate::feed::feedparser;
use crate::library::feedcategory::FeedCategory;
use crate::{
    defs::{self, DATA_CATEGORIES_DIR, DATA_FEED},
    library::feeditem::FeedItem,
};

pub struct LibraryData {
    path: PathBuf,
}

impl LibraryData {
    pub fn new(datapath: &Path) -> LibraryData {
        load_or_create(datapath);
        LibraryData {
            path: PathBuf::from(datapath),
        }
    }

    pub fn feed_exists(&self, slug: &str, category: &str) -> bool {
        let feeddata = self
            .path
            .join(DATA_CATEGORIES_DIR)
            .join(category)
            .join(slug)
            .join(DATA_FEED);
        feeddata.exists()
    }

    pub fn feed_create(&self, feed: &FeedItem) -> color_eyre::Result<()> {
        let feedir = self
            .path
            .join(DATA_CATEGORIES_DIR)
            .join(&feed.category)
            .join(&feed.slug);
        fs::create_dir_all(&feedir)?;

        let feeddata = feedir.join(DATA_FEED);
        let toml_str = toml::to_string(feed)
            .map_err(|e| eyre!("Failed to serialize feed: {}", e))?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&feeddata)
            .map_err(|e| eyre!("Couldn't open file {}: {}", feeddata.display(), e))?;

        file.write_all(toml_str.as_bytes())
            .map_err(|e| eyre!("Failed to write file {}: {}", feeddata.display(), e))
    }

    pub fn generate_categories_tree(&self) -> color_eyre::Result<Vec<FeedCategory>> {
        let mut categories: Vec<FeedCategory> = Vec::new();
        let catpath = self.path.join(DATA_CATEGORIES_DIR);

        for entry in fs::read_dir(catpath)? {
            let path = entry?.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    let cat = FeedCategory {
                        title: String::from(name),
                        feeds: self.load_feeds_from_category(name, path.as_path())?,
                    };

                    categories.push(cat);
                }
            }
        }

        Ok(categories)
    }

    pub fn load_feeds_from_category(
        &self,
        category_name: &str,
        category: &Path,
    ) -> color_eyre::Result<Vec<FeedItem>> {
        let mut feeds = Vec::new();

        for entry in fs::read_dir(category)? {
            let path = entry?.path();
            if path.is_dir() {
                let feedpath = path.join(defs::DATA_FEED);

                if let Ok(file) = std::fs::read_to_string(&feedpath) {
                    let mut feed: FeedItem = match toml::from_str(&file) {
                        Ok(f) => f,
                        Err(e) => {
                            return Err(eyre!("Error: feed file can't be parsed: {}", e));
                        }
                    };

                    feed.category = category_name.to_string();
                    feeds.push(feed);
                }
            }
        }

        Ok(feeds)
    }

    pub fn update_feed_entries(
        &self,
        category: &FeedCategory,
        feed: &FeedItem,
        feedxml: Option<String>,
    ) -> color_eyre::Result<()> {
        let mut feedentries = if let Some(txt) = feedxml {
            feedparser::get_feed_entries_doc(feed, &txt)
        } else {
            feedparser::get_feed_entries(feed)
        }?;

        feedentries.iter_mut().for_each(|e| {
            let item_slug = slugify(&e.title);
            let entrypath = self
                .path
                .join(defs::DATA_CATEGORIES_DIR)
                .join(&category.title)
                .join(&feed.slug)
                .join(format!("{}.md", item_slug));

            e.filepath = entrypath;
        });

        self.update_entries(feed, feedentries)
    }

    fn update_entries(&self, feed: &FeedItem, entries: Vec<FeedEntry>) -> color_eyre::Result<()> {
        for entry in entries.iter().as_ref() {
            // if it exists, it means the entry has been setup already
            if !entry.filepath.exists() {
                let mut file = match OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&entry.filepath)
                {
                    Ok(file) => file,
                    Err(error) => {
                        return Err(eyre!(
                            "Error creating file '{}': {}",
                            entry.filepath.display(),
                            error
                        ));
                    }
                };

                let mut entryclone = (*entry).clone();
                entryclone.text = String::new();

                let entrytext = format!(
                    "---\n{}---\n{}",
                    toml::to_string(&entryclone).unwrap_or(String::new()),
                    &entry.text
                );

                file.write_all(&entrytext.into_bytes())?;
            }
        }

        let mut feed = feed.clone();
        feed.lastupdated = Utc::now();
        self.feed_create(&feed)?;

        Ok(())
    }

    pub fn save_feed_entry(&self, entry: &FeedEntry) -> color_eyre::Result<()> {
        info!("Saving {:?}", entry.filepath);

        let mut file = match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&entry.filepath)
        {
            Ok(file) => file,
            Err(error) => {
                return Err(eyre!(
                    "Error creating file '{}': {}",
                    entry.filepath.display(),
                    error
                ));
            }
        };

        let mut entryclone = (*entry).clone();
        entryclone.text = String::new();

        let entrytext = format!(
            "---\n{}---\n{}",
            toml::to_string(&entryclone).unwrap_or_default(),
            &entry.text,
        );

        file.write_all(&entrytext.into_bytes())?;

        Ok(())
    }

    pub fn load_feed_entries(
        &self,
        category: &FeedCategory,
        item: &FeedItem,
    ) -> color_eyre::Result<Vec<FeedEntry>> {
        let mut entries = vec![];

        let feedir = self
            .path
            .join(DATA_CATEGORIES_DIR)
            .join(&category.title)
            .join(&item.slug);

        for entry in fs::read_dir(&feedir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let contents = std::fs::read_to_string(&path)?;
                let parts: Vec<&str> = contents.split("---").collect();
                if parts.len() < 2 {
                    continue;
                }
                let mut entry: FeedEntry = toml::from_str(parts[1])?;

                entry.filepath = path.clone();

                entry.text = parts[2..].join("---");
                entries.push(entry);
            }
        }

        Ok(entries)
    }

    // TODO: this needs to be cached and only updated every now and then, since it's beeing pretty
    // intensive now
    pub fn get_unread_feed(&self, category: &str, feed_slug: &str) -> color_eyre::Result<u16> {
        let mut unread: u16 = 0;

        let feedir = self
            .path
            .join(DATA_CATEGORIES_DIR)
            .join(category)
            .join(feed_slug);

        for entry in fs::read_dir(feedir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let contents = std::fs::read_to_string(&path)?;
                let parts: Vec<&str> = contents.split("---").collect();
                if parts.len() < 2 {
                    continue;
                }
                let entry: FeedEntry = toml::from_str(parts[1])?;
                if !entry.seen {
                    unread += 1;
                }
            }
        }

        Ok(unread)
    }

    pub fn set_entry_seen(&self, entry: &FeedEntry) {
        if !entry.seen {
            let mut entry = entry.clone();
            entry.seen = true;
            if let Err(e) = self.save_feed_entry(&entry) {
                error!("Couldn't set entry seen: {:?}", e);
            }
        }
    }
}

pub fn load_or_create(path: &Path) {
    let datapath = Path::new(path);
    if !datapath.exists() {
        std::fs::create_dir_all(datapath).expect("Error: Failed to create datapath directory");
        std::fs::create_dir_all(datapath.join(defs::DATA_CATEGORIES_DIR))
            .expect("Error: Failed to create datapath directory");
    }
}
