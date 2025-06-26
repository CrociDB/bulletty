use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span, Text}, widgets::ListItem,
};

use crate::{
    feed::feedentry::FeedEntry,
    library::feedlibrary::FeedLibrary,
    ui::feedtree::{FeedItemInfo, FeedTreeState},
};

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

    pub fn get_items(&self) -> Vec<ListItem> {
        self.entries
            .iter()
            .map(|entry| {
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
                ListItem::new(item_text)
            })
            .collect()
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
