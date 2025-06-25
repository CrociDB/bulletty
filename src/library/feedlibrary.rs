use color_eyre::eyre::eyre;

use crate::{
    defs, feed,
    library::{
        data::{config::Config, librarydata::LibraryData},
        feedcategory::FeedCategory,
        feeditem::FeedItem,
    },
};

pub struct FeedLibrary {
    pub feedcategories: Vec<FeedCategory>,
    pub data: LibraryData,
}

impl FeedLibrary {
    pub fn new() -> FeedLibrary {
        let config_obj = Config::new();
        let data_obj = LibraryData::new(config_obj.datapath.as_ref());

        let categories = match data_obj.generate_categories_tree() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };

        FeedLibrary {
            feedcategories: categories,
            data: data_obj,
        }
    }

    pub fn add_feed(
        &mut self,
        url: &str,
        category: &Option<String>,
    ) -> color_eyre::Result<FeedItem> {
        let feed = feed::feedparser::parse(url)?;

        let category_string = category
            .clone()
            .unwrap_or_else(|| String::from(defs::DATA_CATEGORY_DEFAULT));

        // check if feed already in library
        if self.data.feed_exists(&feed.slug, &category_string) {
            return Err(eyre!("Feed already exists"));
        }

        // then create
        self.data.feed_create(&feed, &category_string)?;

        Ok(feed)
    }
}
