use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Widget},
};

use crate::library::feedlibrary::FeedLibrary;

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

    pub fn set_list_data(&mut self, library: &FeedLibrary) {
        self.listitems.clear();

        self.selected = library.currentselection;
        for (i, title) in library.get_list_data().iter().enumerate() {
            if i == library.currentselection {

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
        let mut list =
            List::new(self.listitems).block(Block::default().title("Feeds").borders(Borders::ALL));

        if !self.enabled {
            let disabled_style = Style::default().fg(Color::Gray).add_modifier(Modifier::DIM);
            list = list.style(disabled_style);
        }

        list.render(area, buf);
    }
}
