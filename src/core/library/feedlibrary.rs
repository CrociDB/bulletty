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
        readlaterentry::ReadLaterEntry,
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
            return Err(eyre!("Feed {:?} already exists", feed.title));
        }

        // then create
        self.data.feed_create(&feed)?;
        Ok(feed)
    }

    pub fn delete_feed(&self, slug: &str, category: &str) -> color_eyre::Result<()> {
        self.data.delete_feed(slug, category)
    }

    pub fn get_feed_entries_by_category(
        &self,
        categorytitle: &str,
    ) -> color_eyre::Result<Vec<FeedEntry>> {
        let mut entries = vec![];

        for category in self.feedcategories.iter() {
            if category.title == categorytitle {
                for feed in category.feeds.iter() {
                    entries.extend(self.data.load_feed_entries(category, feed)?);
                }
            }
        }

        entries.sort_by(|a, b| b.date.cmp(&a.date));
        Ok(entries)
    }

    pub fn get_feed_entries_by_item_slug(&self, slug: &str) -> color_eyre::Result<Vec<FeedEntry>> {
        for category in self.feedcategories.iter() {
            for feed in category.feeds.iter() {
                if feed.slug == slug {
                    let mut entries = self.data.load_feed_entries(category, feed)?;

                    entries.sort_by(|a, b| b.date.cmp(&a.date));
                    return Ok(entries);
                }
            }
        }

        Ok(vec![])
    }

    pub fn start_updater(&mut self) {
        self.updater = Some(Updater::new(self.feedcategories.clone()));
    }

    pub fn update(&mut self) {
        if let Some(updater) = self.updater.as_ref()
            && updater.finished.load(std::sync::atomic::Ordering::Relaxed)
        {
            self.updater = None;
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

    pub fn add_to_read_later(&self, entry: &FeedEntry) -> color_eyre::Result<()> {
        let read_later_entry = ReadLaterEntry::from_feed_entry(entry);
        self.data.add_to_read_later(read_later_entry)
    }

    pub fn remove_from_read_later(&self, file_path: &str) -> color_eyre::Result<()> {
        self.data.remove_from_read_later(file_path)
    }

    pub fn has_read_later_entries(&self) -> bool {
        match self.get_read_later_feed_entries() {
            Ok(entries) => !entries.is_empty(),
            Err(_) => false,
        }
    }

    pub fn is_in_read_later(&self, file_path: &str) -> bool {
        self.data.is_in_read_later(file_path)
    }

    pub fn get_read_later_feed_entries(&self) -> color_eyre::Result<Vec<FeedEntry>> {
        self.data.get_read_later_feed_entries()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::library::feedlibrary::FeedLibrary;
    use crate::core::library::readlaterentry::ReadLaterEntry;

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

    fn setup_test_library_for_matches() -> FeedLibrary {
        let (mut library, _temp_dir) = FeedLibrary::new_for_test();

        let feed1 = crate::core::library::feeditem::FeedItem {
            title: "My Test Feed".to_string(),
            slug: "my-test-feed".to_string(),
            feed_url: "https://mytestfeed/rss".to_string(),
            category: "testing".to_string(),
            ..Default::default()
        };

        let feed2 = crate::core::library::feeditem::FeedItem {
            title: "New sports feed".to_string(),
            slug: "new-sports-feed".to_string(),
            feed_url: "https://sportsfeed/rss".to_string(),
            category: "sports".to_string(),
            ..Default::default()
        };

        let feed3 = crate::core::library::feeditem::FeedItem {
            title: "TechCrunch".to_string(),
            slug: "techcrunch".to_string(),
            feed_url: "https://techcrunch/feed".to_string(),
            category: "tech".to_string(),
            ..Default::default()
        };

        assert!(library.add_feed(feed1.clone()).is_ok());
        assert!(library.add_feed(feed2.clone()).is_ok());
        assert!(library.add_feed(feed3.clone()).is_ok());
        assert!(library.data.feed_exists("techcrunch", "tech"));
        assert!(library.data.feed_exists("new-sports-feed", "sports"));
        assert!(library.data.feed_exists("my-test-feed", "testing"));

        library.feedcategories = library.data.generate_categories_tree().unwrap();

        library
    }

    #[test]
    fn test_exact_slug_match() {
        let library = setup_test_library_for_matches();
        let ident = "my-test-feed";

        let matches = library.get_matching_feeds(ident);
        assert_eq!(
            matches.len(),
            1,
            "Should find exactly one match for exact slug."
        );
        assert_eq!(matches[0].slug, ident);
    }

    #[test]
    fn test_typo_in_title() {
        let library = setup_test_library_for_matches();
        let ident = "new spotfed";

        let matches = library.get_matching_feeds(ident);
        assert_ne!(matches.len(), 0, "Should not be empty for the typo title");
        assert_eq!(matches[0].title, "New sports feed");
    }

    #[test]
    fn test_url_match() {
        let library = setup_test_library_for_matches();
        let ident = "http:/techrunch.com/fed";

        let matches = library.get_matching_feeds(ident);
        assert_eq!(
            matches.len(),
            1,
            "Should find exactly one match for the url"
        );
        assert_eq!(matches[0].feed_url, "https://techcrunch/feed");
    }

    #[test]
    fn test_low_score_no_match() {
        let library = setup_test_library_for_matches();
        let ident = "mytest";

        let matches = library.get_matching_feeds(ident);
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_multiple_matches() {
        let (mut library, _temp_dir) = FeedLibrary::new_for_test();

        let feed1 = crate::core::library::feeditem::FeedItem {
            title: "TechCrunch".to_string(),
            slug: "techcrunch".to_string(),
            feed_url: "https://techcrunch/feed".to_string(),
            category: "General".to_string(),
            ..Default::default()
        };

        let feed2 = crate::core::library::feeditem::FeedItem {
            title: "TechCrunch".to_string(),
            slug: "techcrunch".to_string(),
            feed_url: "https://techcrunch/feed".to_string(),
            category: "tech".to_string(),
            ..Default::default()
        };

        assert!(library.add_feed(feed1.clone()).is_ok());
        assert!(library.add_feed(feed2.clone()).is_ok());
        library.feedcategories = library.data.generate_categories_tree().unwrap();

        let ident = "techcrunch";
        let matches = library.get_matching_feeds(ident);

        assert!(library.data.feed_exists("techcrunch", "tech"));
        assert!(library.data.feed_exists("techcrunch", "General"));

        assert_eq!(matches.len(), 2);
        assert_eq!(
            matches[0].title, matches[1].title,
            "Titles should be equal since both are same feeds."
        );
        assert_ne!(
            matches[0].category, matches[1].category,
            "Category should be different for both the feeds."
        );
    }

    #[test]
    fn test_is_in_read_later() {
        let (library, _tmp) = FeedLibrary::new_for_test();
        // Initially empty
        assert!(!library.is_in_read_later("Test/example/a.md"));

        // Add an entry to read later via data API directly
        let rl = ReadLaterEntry::new("Test/example/a.md".to_string());
        library.data.add_to_read_later(rl).unwrap();

        assert!(!library.is_in_read_later("Test/example/a.md"));
    }
}
