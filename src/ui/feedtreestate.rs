use ratatui::widgets::{ListItem, ListState};

use crate::library::feedlibrary::FeedLibrary;

pub enum FeedItemInfo {
    Category(String),
    Item(String, String),
}

pub struct FeedTreeState {
    pub treeitems: Vec<FeedItemInfo>,
    pub listatate: ListState,
}

impl FeedTreeState {
    pub fn new() -> FeedTreeState {
        let mut state = FeedTreeState {
            treeitems: vec![],
            listatate: ListState::default(),
        };

        state.listatate.select(Some(0));
        state
    }

    pub fn update(&mut self, library: &FeedLibrary) {
        self.treeitems.clear();

        for category in library.feedcategories.iter() {
            self.treeitems
                .push(FeedItemInfo::Category(category.title.clone()));
            for item in category.feeds.iter() {
                self.treeitems
                    .push(FeedItemInfo::Item(item.title.clone(), item.slug.clone()));
            }
        }
    }

    pub fn get_items(&self) -> Vec<ListItem> {
        self.treeitems
            .iter()
            .map(|item| {
                let title = match item {
                    FeedItemInfo::Category(t) => format!("\u{f07c} {}", t),
                    FeedItemInfo::Item(t, _) => format!(" \u{f09e}  {}", t),
                };

                ListItem::new(title.clone())
            })
            .collect()
    }

    pub fn get_selected(&self) -> &FeedItemInfo {
        &self.treeitems[self.listatate.selected().unwrap_or(0)]
    }
}
