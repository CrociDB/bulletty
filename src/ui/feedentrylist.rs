use ratatui::{
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Padding, Widget},
};

use crate::{
    feed::feedentry::FeedEntry,
    library::feedlibrary::FeedLibrary,
    ui::feedtree::{FeedItemInfo, FeedTreeState},
};

// The list state

#[derive(Default)]
pub struct FeedEntryState {
    pub entries: Vec<FeedEntry>,
    pub selected: usize,
    pub previous_selected: String,
}

impl FeedEntryState {
    pub fn update(&mut self, library: &FeedLibrary, treestate: &FeedTreeState) {
        let prev = self.previous_selected.to_string();

        self.entries = match treestate.get_selected() {
            FeedItemInfo::Category(t) => {
                self.previous_selected = t.to_string();
                library.get_feed_entries_by_category(t)
            }
            FeedItemInfo::Item(_, s) => {
                self.previous_selected = s.to_string();
                library.get_feed_entries_by_item_slug(s)
            }
        };

        if prev != self.previous_selected {
            self.selected = 0;
        }
    }

    pub fn selection_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn selection_down(&mut self) {
        self.selected = std::cmp::min(self.selected + 1, self.entries.len() - 1);
    }
}

// The list itself

pub struct FeedEntryList<'a> {
    pub entries: &'a Vec<FeedEntry>,
    pub enabled: bool,
    pub selected: usize,
}

impl<'a> FeedEntryList<'a> {
    pub fn new(
        entry_selected: usize,
        widget_enabled: bool,
        feedentries: &'a Vec<FeedEntry>,
    ) -> FeedEntryList<'a> {
        FeedEntryList {
            entries: feedentries,
            enabled: widget_enabled,
            selected: entry_selected,
        }
    }
}

impl<'a> Widget for FeedEntryList<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let list_items: Vec<ListItem> = self
            .entries
            .iter()
            .enumerate()
            .map(|(id, entry)| {
                let mut item_content_lines: Vec<Line> = Vec::new();

                item_content_lines.push(Line::from(""));
                item_content_lines.push(Line::from(Span::styled(
                    entry.title.clone(),
                    Style::default().bold().underline_color(Color::Blue),
                )));

                item_content_lines.push(Line::from(Span::styled(
                    format!(
                        "\u{f0520} {}",
                        entry.date.with_timezone(&chrono::Local).format("%Y-%m-%d")
                    ),
                    Style::default().italic().dim(),
                )));

                item_content_lines.push(Line::from(Span::styled(
                    format!("{}...", entry.description),
                    Style::default().dim(),
                )));

                item_content_lines.push(Line::from(""));

                let item_text = Text::from(item_content_lines);
                let list_item = ListItem::new(item_text);

                // Highlight the selected item
                if self.enabled && id == self.selected {
                    list_item.style(Style::default().bg(Color::Blue))
                } else {
                    list_item
                }
            })
            .collect();

        let mut list_widget = List::new(list_items).block(
            Block::default()
                .style(Style::default().bg(Color::from_u32(0x3a3a3a)))
                .padding(Padding::new(2, 2, 1, 1)),
        );

        // Render the list widget
        list_widget.render(area, buf);
    }
}
