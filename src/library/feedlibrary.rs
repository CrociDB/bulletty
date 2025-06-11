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
    pub currentselection: usize,
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
            currentselection: 0,
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

    // Navigation: this needs to be moved somehwere else

    pub fn get_list_data(&self) -> Vec<String> {
        let mut items = Vec::<String>::new();
        for category in self.feedcategories.iter() {
            let title = format!(" > {}", category.title);
            items.push(title);

            for feed in category.feeds.iter() {
                items.push(format!("  - {}", feed.title));
            }
        }

        items
    }

    pub fn selection_up(&mut self) {
        if self.currentselection > 0 {
            self.currentselection -= 1;
        }
    }

    pub fn selection_down(&mut self) {
        self.currentselection =
            std::cmp::min(self.currentselection + 1, self.count_total_items() - 1);
    }

    fn count_total_items(&self) -> usize {
        let mut total: usize = 0;
        total += self.feedcategories.len();

        total += self
            .feedcategories
            .iter()
            .map(|c| c.feeds.len())
            .sum::<usize>();

        total
    }
}
