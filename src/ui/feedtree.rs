use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Widget},
};

use crate::library::feedlibrary::FeedLibrary;

pub struct FeedTree<'a> {
    pub selected: usize,
    pub listitems: Vec<ListItem<'a>>,
}

impl<'a> FeedTree<'a> {
    pub fn new() -> FeedTree<'a> {
        FeedTree {
            selected: 0,
            listitems: Vec::new(),
        }
    }

    pub fn set_list_data(&mut self, library: &FeedLibrary) {
        self.listitems.clear();

        self.selected = library.currentselection;
        for (i, title) in library.get_list_data().iter().enumerate() {
            if i == library.currentselection {
                self.listitems
                    .push(ListItem::new(title.clone()).style(Style::new().bg(Color::Yellow)));
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
        let list =
            List::new(self.listitems).block(Block::default().title("Feeds").borders(Borders::ALL));
        list.render(area, buf);
    }
}
