use color_eyre::eyre::eyre;
use fuzzt::algorithms::normalized_levenshtein;
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

#[cfg(test)]
use tempfile::TempDir;

pub struct FeedLibrary {
    pub feedcategories: Vec<FeedCategory>,
    pub data: LibraryData,
    pub updater: Option<Updater>,
}

impl Default for FeedLibrary {
    fn default() -> Self {
        Self::new()
    }
}

impl FeedLibrary {
    pub fn new() -> Self {
        let config_obj = Config::new();
        let data_obj = LibraryData::new(config_obj.datapath.as_ref());

        let categories = match data_obj.generate_categories_tree() {
            Ok(c) => c,
            Err(e) => {
                error!("{}", e);
                std::process::exit(1);
            }
        };

        Self {
            feedcategories: categories,
            data: data_obj,
            updater: None,
        }
    }

    #[cfg(test)]
    pub fn new_for_test() -> (Self, TempDir) {
        let (data_obj, temp_dir) = LibraryData::new_for_test();
        let categories = data_obj.generate_categories_tree().unwrap();
        (
            Self {
                feedcategories: categories,
                data: data_obj,
                updater: None,
            },
            temp_dir,
        )
    }

    pub fn add_feed_from_url(
        &mut self,
        url: &str,
        category: &Option<String>,
    ) -> color_eyre::Result<FeedItem> {
        let mut feed = feed::feedparser::get_feed(url)?;

        feed.category = category
            .clone()
            .unwrap_or_else(|| String::from(defs::DATA_CATEGORY_DEFAULT));

        self.add_feed(feed)
    }

    pub fn add_feed(&mut self, feed: FeedItem) -> color_eyre::Result<FeedItem> {
        // check if feed already in library
        if self.data.feed_exists(&feed.slug, &feed.category) {
            return Err(eyre!("Feed already exists"));
        }

        // then create
        self.data.feed_create(&feed)?;
        Ok(feed)
    }

    pub fn delete_feed(&self, slug: &str, category: &str) -> color_eyre::Result<()> {
        self.data.delete_feed(slug, category)
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

    pub fn get_matching_feeds(&self, ident: &str) -> Vec<&FeedItem> {
        let mut matching_vec: Vec<&FeedItem> = Vec::new();

        // Check for matching feeds and push to vec
        for category in self.feedcategories.iter() {
            for feed in category.feeds.iter() {
                let slug_score = normalized_levenshtein(&feed.slug, ident);
                let title_score = normalized_levenshtein(&feed.title, ident);
                let url_score = normalized_levenshtein(&feed.feed_url, ident);
                let max_score = slug_score.max(title_score).max(url_score);

                if max_score > 0.6 {
                    matching_vec.push(feed);
                }
            }
        }

        matching_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_delete_feed() {
        // 1. Create a FeedLibrary that uses a temporary, in-memory database
        let (mut library, _temp_dir) = FeedLibrary::new_for_test();

        // 2. Create a dummy FeedItem to add
        let feed_to_add = crate::core::library::feeditem::FeedItem {
            title: "My Test Feed".to_string(),
            slug: "my-test-feed".to_string(),
            category: "testing".to_string(),
            ..Default::default()
        };

        // 3. Add the feed to the library and verify
        assert!(library.add_feed(feed_to_add.clone()).is_ok());
        assert!(library.data.feed_exists("my-test-feed", "testing"));

        // 4. Delete the feed and verify
        assert!(
            library
                .delete_feed(&feed_to_add.slug, &feed_to_add.category)
                .is_ok()
        );
        assert!(!library.data.feed_exists("my-test-feed", "testing"));
    }
}
