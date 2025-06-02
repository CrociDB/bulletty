use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Widget},
};

use crate::library::feedlibrary::FeedLibrary;

pub struct FeedTree<'a> {
    selected: usize,
    library: &'a FeedLibrary,
}

impl<'a> FeedTree<'a> {
    pub fn new(lib: &'a FeedLibrary) -> FeedTree<'a> {
        FeedTree {
            selected: 0,
            library: lib,
        }
    }
}

impl<'a> Widget for FeedTree<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mut items = Vec::<ListItem>::new();

        for (i, item) in self.library.feedcategories.iter().enumerate() {
            if i == self.selected {
                items.push(ListItem::new(item.title.clone()).style(Style::new().bg(Color::Yellow)));
            } else {
                items.push(ListItem::new(item.title.clone()));
            }
        }

        let list = List::new(items).block(Block::default().title("Feeds").borders(Borders::ALL));
        list.render(area, buf);
    }
}
