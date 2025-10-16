use ratatui::widgets::{ListItem, ListState};
use tracing::error;

use crate::core::library::feedlibrary::FeedLibrary;

pub enum FeedItemInfo {
    /// Represents the category title
    Category(String),
    /// Represents an item in the feed tree with a title, categore, and slug
    Item(String, String, String),
    /// Represents a separator in the menu
    Separator,
    /// Represents the Read Later category
    ReadLater,
}

pub struct FeedTreeState {
    pub treeitems: Vec<FeedItemInfo>,
    pub listatate: ListState,
}

impl Default for FeedTreeState {
    fn default() -> Self {
        Self::new()
    }
}

impl FeedTreeState {
    pub fn new() -> Self {
        Self {
            treeitems: vec![],
            listatate: ListState::default().with_selected(Some(0)),
        }
    }

    pub fn update(&mut self, library: &FeedLibrary) {
        self.treeitems.clear();

        for category in library.feedcategories.iter() {
            self.treeitems
                .push(FeedItemInfo::Category(category.title.clone()));
            for item in category.feeds.iter() {
                self.treeitems.push(FeedItemInfo::Item(
                    item.title.clone(),
                    category.title.clone(),
                    item.slug.clone(),
                ));
            }
        }

        // display Read Later section if it has entries
        if library.has_read_later_entries() {
            self.treeitems.push(FeedItemInfo::Separator);
            self.treeitems.push(FeedItemInfo::ReadLater);
        }
    }

    pub fn get_items(&self, library: &FeedLibrary) -> Vec<ListItem<'_>> {
        self.treeitems
            .iter()
            .map(|item| {
                let title = match item {
                    FeedItemInfo::Category(t) => format!("\u{f07c} {t}"),
                    FeedItemInfo::Item(t, c, s) => {
                        if let Ok(unread) = library.data.get_unread_feed(c, s) {
                            if unread > 0 {
                                format!(" \u{f09e}  {t} ({unread})")
                            } else {
                                format!(" \u{f09e}  {t}")
                            }
                        } else {
                            error!("Couldn't get unread feed entries for '{}'", t);
                            format!(" \u{f09e}  {t}")
                        }
                    }
                    FeedItemInfo::Separator => "".to_string(),
                    FeedItemInfo::ReadLater => {
                        if let Ok(count) = library.get_read_later_feed_entries() {
                            format!("\u{f02d} Read Later ({})", count.len())
                        } else {
                            "\u{f02d} Read Later".to_string()
                        }
                    }
                };

                ListItem::new(title.clone())
            })
            .collect()
    }

    pub fn get_selected(&self) -> Option<&FeedItemInfo> {
        if !self.treeitems.is_empty() {
            Some(&self.treeitems[self.listatate.selected().unwrap_or(0)])
        } else {
            None
        }
    }

    pub fn select_next(&mut self) {
        if self.treeitems.is_empty() {
            return;
        }

        if self.listatate.selected().unwrap_or(0) < self.treeitems.len().saturating_sub(1) {
            self.listatate.select_next();
        }
    }

    pub fn select_previous(&mut self) {
        if self.treeitems.is_empty() {
            return;
        }

        if self.listatate.selected().unwrap_or(0) > 0 {
            self.listatate.select_previous();
        }
    }

    pub fn select_first(&mut self) {
        if self.treeitems.is_empty() {
            return;
        }

        self.listatate.select_first();
    }

    pub fn select_last(&mut self) {
        if self.treeitems.is_empty() {
            return;
        }

        self.listatate
            .select(Some(self.treeitems.len().saturating_sub(1)));
    }
}
