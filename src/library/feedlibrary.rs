use color_eyre::eyre::eyre;

use crate::{
    defs,
    feed::{self, feedentry::FeedEntry},
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

    pub fn get_feed_entries_by_category(&self, categorytitle: &str) -> Vec<FeedEntry> {
        let mut entries = vec![];

        for category in self.feedcategories.iter() {
            if category.title == categorytitle {
                for feed in category.feeds.iter() {
                    entries.extend(match self.data.load_feed_entries(category, feed) {
                        Ok(entries) => entries,
                        Err(e) => {
                            println!("Error: {:?}", e);
                            vec![]
                        }
                    });
                }
            }
        }

        entries.sort_by(|a, b| b.date.cmp(&a.date));
        entries
    }

    pub fn get_feed_entries_by_item_slug(&self, slug: &str) -> Vec<FeedEntry> {
        for category in self.feedcategories.iter() {
            for feed in category.feeds.iter() {
                if feed.slug == slug {
                    let mut entries = match self.data.load_feed_entries(category, feed) {
                        Ok(entries) => entries,
                        Err(e) => {
                            println!("Error: {:?}", e);
                            vec![]
                        }
                    };

                    entries.sort_by(|a, b| b.date.cmp(&a.date));
                    return entries;
                }
            }
        }

        vec![]
    }
}
