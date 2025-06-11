use color_eyre::{Section, SectionExt, eyre::Report, eyre::eyre};

use crate::{
    feedparser,
    library::{
        data::{config::Config, data::Data},
        feedcategory::FeedCategory,
        feeditem::FeedItem,
    },
};

pub struct FeedLibrary {
    pub feedcategories: Vec<FeedCategory>,
    pub currentselection: usize,
    pub config: Config,
    pub data: Data,
}

impl FeedLibrary {
    pub fn new() -> FeedLibrary {
        let config_obj = Config::new();
        let data_obj = Data::new(config_obj.datapath.as_ref());

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
            config: config_obj,
            data: data_obj,
        }
    }

    pub fn add_feed(&mut self, url: &str) -> color_eyre::Result<FeedItem> {
        let feed = feedparser::feedparser::parse(url)?;

        // check if feed already in library
        if self.data.feed_exists(&feed.slug) {
            return Err(eyre!("Feed already exists"));
        }

        // then create
        self.data.feed_create(&feed)?;

        Ok(feed)
    }

    // Navigation: this needs to be moved somehwere else

    pub fn get_list_data(&self) -> Vec<String> {
        let mut items = Vec::<String>::new();
        for item in self.feedcategories.iter() {
            let title = format!(" > {}", item.title);
            items.push(title);
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
        self.feedcategories.len()
    }
}
