use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Padding, Widget},
};

use crate::library::feedlibrary::FeedLibrary;

// FeedTreeState

pub enum FeedItemInfo {
    Category(String),
    Item(String, String),
}

#[derive(Default)]
pub struct FeedTreeState {
    pub treeitems: Vec<FeedItemInfo>,
    pub selected: usize,
}

impl FeedTreeState {
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

    pub fn get_selected(&self) -> &FeedItemInfo {
        &self.treeitems[self.selected]
    }

    pub fn selection_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn selection_down(&mut self) {
        self.selected = std::cmp::min(self.selected + 1, self.treeitems.len() - 1);
    }
}

// FeedTree

pub struct FeedTree<'a> {
    pub selected: usize,
    pub listitems: Vec<ListItem<'a>>,
    pub enabled: bool,
}

impl<'a> FeedTree<'a> {
    pub fn new() -> FeedTree<'a> {
        FeedTree {
            selected: 0,
            listitems: Vec::new(),
            enabled: true,
        }
    }

    pub fn set_list_data(&mut self, state: &FeedTreeState) {
        self.listitems.clear();

        self.selected = state.selected;

        for (i, item) in state.treeitems.iter().enumerate() {
            let title = match item {
                FeedItemInfo::Category(t) => format!("\u{f07c} {}", t),
                FeedItemInfo::Item(t, _) => format!(" \u{f09e}  {}", t),
            };

            if i == self.selected {
                let color = if self.enabled {
                    Color::Yellow
                } else {
                    Color::Gray
                };

                self.listitems
                    .push(ListItem::new(title.clone()).style(Style::new().bg(color)));
            } else {
                self.listitems.push(ListItem::new(title.clone()));
            }
        }
    }
}

impl<'a> Widget for FeedTree<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mut list = List::new(self.listitems).block(
            Block::default()
                .style(Style::default().bg(Color::from_u32(0x262626)))
                .padding(Padding::new(2, 2, 2, 2)),
        );

        if !self.enabled {
            let disabled_style = Style::default().fg(Color::Gray).add_modifier(Modifier::DIM);
            list = list.style(disabled_style);
        }

        list.render(area, buf);
    }
}
