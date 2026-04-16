use std::collections::HashMap;

use ratatui::widgets::{ListItem, ListState};

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
    last_generation: u64,
    unread_counts: HashMap<(String, String), u16>,
    read_later_count: usize,
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
            last_generation: u64::MAX,
            unread_counts: HashMap::new(),
            read_later_count: 0,
        }
    }

    pub fn update(&mut self, library: &mut FeedLibrary) {
        if library.generation == self.last_generation {
            return;
        }
        self.last_generation = library.generation;

        self.treeitems.clear();
        self.unread_counts.clear();

        for category in library.feedcategories.iter() {
            self.treeitems
                .push(FeedItemInfo::Category(category.title.clone()));
            for item in category.feeds.iter() {
                self.treeitems.push(FeedItemInfo::Item(
                    item.title.clone(),
                    category.title.clone(),
                    item.slug.clone(),
                ));

                if let Ok(count) = library.data.get_unread_feed(&category.title, &item.slug) {
                    self.unread_counts
                        .insert((category.title.clone(), item.slug.clone()), count);
                }
            }
        }

        // display Read Later section if it has entries
        match library.get_read_later_feed_entries() {
            Ok(entries) if !entries.is_empty() => {
                self.read_later_count = entries.len();
                self.treeitems.push(FeedItemInfo::Separator);
                self.treeitems.push(FeedItemInfo::ReadLater);
            }
            _ => {
                self.read_later_count = 0;
            }
        }
    }

    pub fn get_items(&self) -> Vec<ListItem<'_>> {
        self.treeitems
            .iter()
            .map(|item| {
                let title = match item {
                    FeedItemInfo::Category(t) => format!("\u{f07c} {t}"),
                    FeedItemInfo::Item(t, c, s) => {
                        let unread = self
                            .unread_counts
                            .get(&(c.clone(), s.clone()))
                            .copied()
                            .unwrap_or(0);
                        if unread > 0 {
                            format!(" \u{f09e}  {t} ({unread})")
                        } else {
                            format!(" \u{f09e}  {t}")
                        }
                    }
                    FeedItemInfo::Separator => "".to_string(),
                    FeedItemInfo::ReadLater => {
                        if self.read_later_count > 0 {
                            format!("\u{f02d} Read Later ({})", self.read_later_count)
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
            let idx = self.listatate.selected().unwrap_or(0);
            let clamped = idx.min(self.treeitems.len().saturating_sub(1));
            Some(&self.treeitems[clamped])
        } else {
            None
        }
    }

    pub fn select_next(&mut self) {
        if self.treeitems.is_empty() {
            return;
        }

        let selected = self.listatate.selected().unwrap_or(0);
        if selected < self.treeitems.len().saturating_sub(1) {
            self.listatate.select_next();

            if self.is_selected_separator() {
                self.select_next();
            }
        }
    }

    pub fn select_previous(&mut self) {
        if self.treeitems.is_empty() {
            return;
        }

        let selected = self.listatate.selected().unwrap_or(0);
        if selected >= self.treeitems.len() {
            self.listatate
                .select(Some(self.treeitems.len().saturating_sub(1)));
        }

        let selected = self.listatate.selected().unwrap_or(0);

        if selected > 0 {
            self.listatate.select_previous();
            if self.is_selected_separator() {
                self.select_previous();
            }
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

    pub fn select_next_category(&mut self) {
        let current = self.listatate.selected().unwrap_or(0);
        for (i, item) in self.treeitems.iter().enumerate().skip(current + 1) {
            if matches!(item, FeedItemInfo::Category(_) | FeedItemInfo::ReadLater) {
                self.listatate.select(Some(i));
                return;
            }
        }
    }

    pub fn select_previous_category(&mut self) {
        let current = self.listatate.selected().unwrap_or(0);
        for (i, item) in self.treeitems.iter().enumerate().take(current).rev() {
            if matches!(item, FeedItemInfo::Category(_) | FeedItemInfo::ReadLater) {
                self.listatate.select(Some(i));
                return;
            }
        }
    }

    fn is_selected_separator(&self) -> bool {
        if let Some(index) = self.listatate.selected() {
            index < self.treeitems.len() && matches!(self.treeitems[index], FeedItemInfo::Separator)
        } else {
            false
        }
    }
}
