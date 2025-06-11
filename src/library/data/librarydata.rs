use std::io::Write;
use std::{
    fs::{self, OpenOptions},
    path::{Path, PathBuf},
};

use color_eyre::eyre::eyre;

use crate::library::feedcategory::{FeedCategory};
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
        let feedir = self
            .path
            .join(DATA_CATEGORIES_DIR)
            .join(category)
            .join(slug);
        if feedir.exists() {
            let feeddata = feedir.join(DATA_FEED);
            return feeddata.exists();
        }

        false
    }

    pub fn feed_create(&self, feed: &FeedItem, category: &str) -> color_eyre::Result<()> {
        let feedir = self
            .path
            .join(DATA_CATEGORIES_DIR)
            .join(category)
            .join(&feed.slug);
        fs::create_dir_all(&feedir)?;

        let feeddata = feedir.join(DATA_FEED);

        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&feeddata)
        {
            Ok(mut file) => {
                if let Err(e) = file.write_all(&toml::to_string(&feed).unwrap().into_bytes()) {
                    Err(eyre!("Failed to write file {}: {}", feeddata.display(), e))
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(eyre!("Couldn't open file {}: {}", feeddata.display(), e)),
        }
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
                        feeds: self.load_feeds_from_category(path.as_path())?,
                    };

                    categories.push(cat);
                }
            }
        }

        Ok(categories)
    }

    pub fn load_feeds_from_category(&self, category: &Path) -> color_eyre::Result<Vec<FeedItem>> {
        let mut feeds = Vec::new();

        for entry in fs::read_dir(category)? {
            let path = entry?.path();
            if path.is_dir() {
                let feedpath = path.join(defs::DATA_FEED);

                if let Ok(file) = std::fs::read_to_string(&feedpath) {
                    let feed = match toml::from_str(&file) {
                        Ok(f) => f,
                        Err(e) => {
                            return Err(eyre!("Error: feed file can't be parsed: {}", e));
                        }
                    };

                    feeds.push(feed);
                }
            }
        }

        Ok(feeds)
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
