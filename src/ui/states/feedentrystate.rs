use std::{cell::RefCell, rc::Rc};

use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{ListItem, ListState},
};
use tracing::error;

use crate::{
    core::{
        feed::feedentry::FeedEntry,
        library::{feedlibrary::FeedLibrary, settings::theme::Theme},
    },
    ui::states::feedtreestate::{FeedItemInfo, FeedTreeState},
};

pub struct FeedEntryState {
    pub entries: Vec<FeedEntry>,
    pub listatate: ListState,
    pub previous_selected: String,
    pub library: Option<Rc<RefCell<FeedLibrary>>>,
    theme: Theme,
}

impl Default for FeedEntryState {
    fn default() -> Self {
        Self::new()
    }
}

impl FeedEntryState {
    pub fn new() -> Self {
        Self {
            entries: vec![],
            listatate: ListState::default().with_selected(Some(0)),
            previous_selected: String::new(),
            library: None,
            theme: Theme::default(),
        }
    }

    pub fn update(&mut self, library: &mut FeedLibrary, treestate: &FeedTreeState) {
        let prev = self.previous_selected.to_string();
        self.theme = library.settings.get_theme().unwrap().clone();

        self.entries = match treestate.get_selected() {
            Some(FeedItemInfo::Category(t)) => {
                self.previous_selected = t.to_string();
                match library.get_feed_entries_by_category(t) {
                    Ok(entries) => entries,
                    Err(e) => {
                        error!("Error getting feed entries by category: {:?}", e);
                        vec![]
                    }
                }
            }
            Some(FeedItemInfo::Item(_, _, s)) => {
                self.previous_selected = s.to_string();
                match library.get_feed_entries_by_item_slug(s) {
                    Ok(entries) => entries,
                    Err(e) => {
                        error!("Error getting feed entries by item slug: {:?}", e);
                        vec![]
                    }
                }
            }
            Some(FeedItemInfo::ReadLater) => {
                self.previous_selected = "read_later".to_string();
                match library.get_read_later_feed_entries() {
                    Ok(entries) => entries,
                    Err(e) => {
                        error!("Error getting Read Later entries: {:?}", e);
                        vec![]
                    }
                }
            }
            _ => vec![],
        };

        if prev != self.previous_selected {
            self.listatate.select_first();
        }
    }

    pub fn get_items(&self) -> Vec<ListItem<'_>> {
        self.entries
            .iter()
            .map(|entry| {
                let mut item_content_lines: Vec<Line> = Vec::new();

                item_content_lines.push(Line::from(""));

                let read_later_icon =
                    if self.is_in_read_later(entry.filepath.to_str().unwrap_or_default()) {
                        " \u{f02d}" // read later icon
                    } else {
                        ""
                    };

                // Title
                if !entry.seen {
                    item_content_lines.push(Line::from(Span::styled(
                        format!(" \u{f1ea} {}{} \u{e3e3}", entry.title, read_later_icon),
                        Style::default()
                            .bold()
                            .fg(Color::from_u32(self.theme.base[9])),
                    )));
                } else {
                    item_content_lines.push(Line::from(Span::styled(
                        format!(" \u{f1ea} {}{}", entry.title, read_later_icon),
                        Style::default()
                            .bold()
                            .fg(Color::from_u32(self.theme.base[6])),
                    )));
                };

                // Date
                item_content_lines.push(Line::from(Span::styled(
                    format!(
                        " \u{f0520} {} | \u{f09e} {}",
                        entry.date.with_timezone(&chrono::Local).format("%Y-%m-%d"),
                        entry.author
                    ),
                    Style::default().fg(Color::from_u32(self.theme.base[5])),
                )));

                // Description
                item_content_lines.push(Line::from(Span::styled(
                    format!(" {}...", entry.description),
                    Style::default().fg(Color::from_u32(self.theme.base[4])),
                )));

                item_content_lines.push(Line::from(""));

                let item_text = Text::from(item_content_lines);
                ListItem::new(item_text)
            })
            .collect()
    }

    pub fn get_selected(&self) -> Option<FeedEntry> {
        match self.listatate.selected() {
            None => None,
            Some(selected) => {
                if selected < self.entries.len() {
                    Some(self.entries[selected].clone())
                } else {
                    None
                }
            }
        }
    }

    pub fn set_current_read(&mut self) {
        if let Some(selected) = self.listatate.selected()
            && selected < self.entries.len()
        {
            self.entries[selected].seen = true;
        }
    }

    pub fn select_next(&mut self) {
        if self.entries.is_empty() {
            return;
        }

        if self.listatate.selected().unwrap_or(0) < self.entries.len().saturating_sub(1) {
            self.listatate.select_next();
        }
    }

    pub fn select_previous(&mut self) {
        if self.entries.is_empty() {
            return;
        }

        if self.listatate.selected().unwrap_or(0) > 0 {
            self.listatate.select_previous();
        }
    }

    pub fn select_first(&mut self) {
        if self.entries.is_empty() {
            return;
        }

        self.listatate.select_first();
    }

    pub fn select_last(&mut self) {
        if self.entries.is_empty() {
            return;
        }

        self.listatate
            .select(Some(self.entries.len().saturating_sub(1)));
    }

    pub fn scroll_max(&self) -> usize {
        self.entries.len().saturating_sub(1)
    }

    pub fn scroll(&self) -> usize {
        self.listatate.selected().unwrap_or(0)
    }

    fn is_in_read_later(&self, file_path: &str) -> bool {
        if let Some(library) = &self.library {
            library.borrow_mut().is_in_read_later(file_path)
        } else {
            false
        }
    }
}
