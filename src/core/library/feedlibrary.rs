use color_eyre::eyre::eyre;
use tracing::error;

use crate::{
    app::AppWorkStatus,
    core::defs,
    core::feed::{self, feedentry::FeedEntry},
    core::library::{
        data::{config::Config, librarydata::LibraryData},
        feedcategory::FeedCategory,
        feeditem::FeedItem,
        updater::Updater,
    },
};

pub struct FeedLibrary {
    pub feedcategories: Vec<FeedCategory>,
    pub data: LibraryData,
    pub updater: Option<Updater>,
}

impl FeedLibrary {
    pub fn new() -> FeedLibrary {
        let config_obj = Config::new();
        let data_obj = LibraryData::new(config_obj.datapath.as_ref());

        let categories = match data_obj.generate_categories_tree() {
            Ok(c) => c,
            Err(e) => {
                error!("{}", e);
                std::process::exit(1);
            }
        };

        FeedLibrary {
            feedcategories: categories,
            data: data_obj,
            updater: None,
        }
    }

    pub fn add_feed(
        &mut self,
        url: &str,
        category: &Option<String>,
    ) -> color_eyre::Result<FeedItem> {
        let mut feed = feed::feedparser::get_feed(url)?;

        feed.category = category
            .clone()
            .unwrap_or_else(|| String::from(defs::DATA_CATEGORY_DEFAULT));

        // check if feed already in library
        if self.data.feed_exists(&feed.slug, &feed.category) {
            return Err(eyre!("Feed already exists"));
        }

        // then create
        self.data.feed_create(&feed)?;
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
                            error!("{:?}", e);
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
                            error!("{:?}", e);
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

    pub fn start_updater(&mut self) {
        self.updater = Some(Updater::new(self.feedcategories.clone()));
    }

    pub fn update(&mut self) {
        if let Some(updater) = self.updater.as_ref() {
            if updater.finished.load(std::sync::atomic::Ordering::Relaxed) {
                self.updater = None;
            }
        }
    }

    pub fn get_update_status(&self) -> AppWorkStatus {
        if let Some(updater) = self.updater.as_ref() {
            let total: f32 = self
                .feedcategories
                .iter()
                .map(|cat| cat.feeds.len() as f32)
                .sum();

            AppWorkStatus::Working(
                1.0_f32.min(
                    updater
                        .total_completed
                        .load(std::sync::atomic::Ordering::Relaxed) as f32
                        / total,
                ),
                updater.last_completed.lock().unwrap().to_string(),
            )
        } else {
            AppWorkStatus::None
        }
    }
}
