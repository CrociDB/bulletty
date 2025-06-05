use std::io::Write;
use std::{
    fs::{self, OpenOptions},
    path::{Path, PathBuf},
};

use color_eyre::eyre::eyre;

use crate::{
    defs::{self, DATA_CATEGORIES_DIR, DATA_FEED},
    library::feeditem::FeedItem,
};

pub struct Data {
    path: PathBuf,
}

impl Data {
    pub fn new(datapath: &Path) -> Data {
        load_or_create(datapath);
        Data {
            path: PathBuf::from(datapath),
        }
    }

    pub fn feed_exists(&self, slug: &str) -> bool {
        let feedir = self.path.join(DATA_CATEGORIES_DIR).join(slug);
        if feedir.exists() {
            let feeddata = feedir.join(DATA_FEED);
            return feeddata.exists();
        }

        false
    }

    pub fn feed_create(&self, feed: &FeedItem) -> color_eyre::Result<()> {
        let feedir = self.path.join(DATA_CATEGORIES_DIR).join(&feed.slug);
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
            Err(e) => {
                Err(eyre!("Couldn't open file {}: {}", feeddata.display(), e))
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
