use ratatui::{
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Padding, Widget},
};

use crate::{feed::feedentry::FeedEntry, library::feedlibrary::FeedLibrary, ui::feedtree::{FeedItemInfo, FeedTreeState}};

// The list state

#[derive(Default)]
pub struct FeedEntryState {
    pub entries: Vec<FeedEntry>,
    pub selected: usize,
}

impl FeedEntryState {
    pub fn update(&mut self, library: &FeedLibrary, treestate: &FeedTreeState) {

        self.entries = match treestate.get_selected() {
            FeedItemInfo::Category(t) => library.get_feed_entries_by_category(t),
            FeedItemInfo::Item(_, s) => library.get_feed_entries_by_item_slug(s),
        }

    }
}

// The list itself

pub struct FeedEntryList<'a> {
    pub entries: &'a Vec<FeedEntry>,
    pub selected: bool,
}

impl<'a> FeedEntryList<'a> {
    pub fn new(sel: bool, feedentries: &'a Vec<FeedEntry>) -> FeedEntryList<'a> {
        FeedEntryList {
            entries: feedentries,
            selected: sel,
        }
    }
}

impl<'a> Widget for FeedEntryList<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let list_items: Vec<ListItem> = self
            .entries
            .iter()
            .map(|entry| {
                let mut item_content_lines: Vec<Line> = Vec::new();

                // Title - Bold
                item_content_lines.push(Line::from(Span::styled(
                    entry.title.clone(),
                    Style::default().bold(),
                )));

                // Date - Italics
                item_content_lines.push(Line::from(Span::styled(
                    entry.date.clone(),
                    Style::default().italic(),
                )));

                // Text - Approximately 3 lines
                // This takes up to the first 3 lines from the entry's text.
                let text_lines: Vec<Line> = entry
                    .text
                    .lines()
                    .take(3)
                    .map(|s| Line::from(s.to_string()))
                    .collect();
                item_content_lines.extend(text_lines);

                let item_text = Text::from(item_content_lines);
                let list_item = ListItem::new(item_text);

                // Highlight the selected item
                // if i == self.selected {
                //     list_item.style(Style::default().bg(Color::Blue)) // Example highlight
                // } else {
                //     list_item
                // }
                list_item
            })
            .collect();

        let mut list_widget = List::new(list_items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Feed Entries")
                .padding(Padding::uniform(2)),
        );

        if !self.selected {
            let disabled_style = Style::default().fg(Color::Gray).add_modifier(Modifier::DIM);
            list_widget = list_widget.style(disabled_style);
        }

        // Render the list widget
        list_widget.render(area, buf);
    }
}
