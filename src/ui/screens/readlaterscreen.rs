use std::{cell::RefCell, rc::Rc};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        Block, List, ListItem, ListState, Padding, Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
};

use crate::{
    app::AppWorkStatus,
    core::{
        library::{feedlibrary::FeedLibrary, readlaterentry::ReadLaterEntry},
        ui::appscreen::{AppScreen, AppScreenEvent},
    },
    ui::screens::helpdialog::HelpDialog,
    ui::screens::readerscreen::ReaderScreen,
};

pub struct ReadLaterScreen {
    library: Rc<RefCell<FeedLibrary>>,
    entries: Vec<ReadLaterEntry>,
    list_state: ListState,
    scroll_max: usize,
}

impl ReadLaterScreen {
    pub fn new(library: Rc<RefCell<FeedLibrary>>) -> Self {
        let entries = library
            .borrow()
            .get_read_later_entries()
            .unwrap_or_default();
        let scroll_max = entries.len().saturating_sub(1);

        Self {
            library,
            entries,
            list_state: ListState::default().with_selected(Some(0)),
            scroll_max,
        }
    }

    fn build_entries_for_reader(&self) -> Vec<crate::core::feed::feedentry::FeedEntry> {
        let mut entries: Vec<crate::core::feed::feedentry::FeedEntry> = Vec::new();

        for rl in self.entries.iter() {
            if let Some(rel_path) = &rl.file_path {
                let full_path = self
                    .library
                    .borrow()
                    .data
                    .path
                    .join(crate::core::defs::DATA_CATEGORIES_DIR)
                    .join(rel_path);

                if let Ok(contents) = std::fs::read_to_string(&full_path) {
                    let parts: Vec<&str> = contents.split("---").collect();
                    if parts.len() >= 2 {
                        if let Ok(mut fe) =
                            toml::from_str::<crate::core::feed::feedentry::FeedEntry>(parts[1])
                        {
                            fe.filepath = full_path.clone();
                            fe.text = parts[2..].join("---");
                            entries.push(fe);
                            continue;
                        }
                    }
                }
            }

            // Fallback to synthesized entry
            entries.push(rl.to_feed_entry());
        }

        entries
    }

    fn get_selected_entry(&self) -> Option<&ReadLaterEntry> {
        if let Some(selected) = self.list_state.selected() {
            self.entries.get(selected)
        } else {
            None
        }
    }

    fn select_next(&mut self) {
        if self.entries.is_empty() {
            return;
        }

        if self.list_state.selected().unwrap_or(0) < self.entries.len().saturating_sub(1) {
            self.list_state.select_next();
        }
    }

    fn select_previous(&mut self) {
        if self.entries.is_empty() {
            return;
        }

        if self.list_state.selected().unwrap_or(0) > 0 {
            self.list_state.select_previous();
        }
    }

    fn select_first(&mut self) {
        if self.entries.is_empty() {
            return;
        }

        self.list_state.select_first();
    }

    fn select_last(&mut self) {
        if self.entries.is_empty() {
            return;
        }

        self.list_state.select_last();
    }

    fn remove_selected(&mut self) -> Result<()> {
        if let Some(entry) = self.get_selected_entry() {
            self.library.borrow().remove_from_read_later(&entry.url)?;

            self.entries = self
                .library
                .borrow()
                .get_read_later_entries()
                .unwrap_or_default();
            self.scroll_max = self.entries.len().saturating_sub(1);

            if self.list_state.selected().unwrap_or(0) >= self.entries.len() {
                if !self.entries.is_empty() {
                    self.list_state.select(Some(self.entries.len() - 1));
                } else {
                    self.list_state.select(None);
                }
            }
        }

        Ok(())
    }

    fn open_external_url(&self, url: &str) -> Result<AppScreenEvent> {
        if let Err(e) = open::that(url) {
            tracing::error!("Failed to open URL: {:?}", e);
        }
        Ok(AppScreenEvent::None)
    }

    fn get_items(&self) -> Vec<ListItem<'static>> {
        self.entries
            .iter()
            .map(|entry| {
                let title = if entry.title.len() > 60 {
                    format!("{}...", &entry.title[..57])
                } else {
                    entry.title.clone()
                };

                let source_info = if let (Some(_feed), Some(category)) =
                    (&entry.source_feed, &entry.source_category)
                {
                    format!(" ({})", category)
                } else if let Some(feed) = &entry.source_feed {
                    format!(" ({})", feed)
                } else {
                    String::new()
                };

                let date_str = entry.date_added.format("%Y-%m-%d %H:%M").to_string();

                ListItem::new(vec![
                    Line::from(vec![
                        Span::styled(title, Style::default().fg(Color::White)),
                        Span::styled(source_info, Style::default().fg(Color::Gray)),
                    ]),
                    Line::from(vec![Span::styled(
                        date_str,
                        Style::default().fg(Color::DarkGray),
                    )]),
                ])
            })
            .collect()
    }
}

impl AppScreen for ReadLaterScreen {
    fn start(&mut self) {}

    fn quit(&mut self) {}

    fn pause(&mut self) {}

    fn unpause(&mut self) {}

    fn render(&mut self, frame: &mut ratatui::Frame, area: Rect) {
        let chunks =
            Layout::horizontal([Constraint::Percentage(100), Constraint::Length(1)]).split(area);

        let list_widget = List::new(self.get_items())
            .block(
                Block::default()
                    .title("Read Later")
                    .style(Style::default().bg(Color::from_u32(0x262626)))
                    .padding(Padding::new(2, 2, 2, 2)),
            )
            .highlight_style(Style::default().bg(Color::from_u32(0x514537)));

        let mut list_state = self.list_state.clone();
        frame.render_stateful_widget(list_widget, chunks[0], &mut list_state);
        self.list_state = list_state;

        // Render scrollbar
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let mut scrollbar_state = ScrollbarState::default()
            .content_length(self.entries.len())
            .position(self.list_state.selected().unwrap_or(0));

        frame.render_stateful_widget(scrollbar, chunks[1], &mut scrollbar_state);
    }

    fn handle_events(&mut self) -> Result<AppScreenEvent> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.handle_keypress(key),
            _ => Ok(AppScreenEvent::None),
        }
    }

    fn handle_keypress(&mut self, key: crossterm::event::KeyEvent) -> Result<AppScreenEvent> {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                Ok(AppScreenEvent::ExitState)
            }
            (_, KeyCode::Down | KeyCode::Char('j')) => {
                self.select_next();
                Ok(AppScreenEvent::None)
            }
            (_, KeyCode::Up | KeyCode::Char('k')) => {
                self.select_previous();
                Ok(AppScreenEvent::None)
            }
            (_, KeyCode::Home | KeyCode::Char('g')) => {
                self.select_first();
                Ok(AppScreenEvent::None)
            }
            (_, KeyCode::End | KeyCode::Char('G')) => {
                self.select_last();
                Ok(AppScreenEvent::None)
            }
            (_, KeyCode::Char('o')) => {
                if let Some(entry) = self.get_selected_entry() {
                    self.open_external_url(&entry.url)
                } else {
                    Ok(AppScreenEvent::None)
                }
            }
            (_, KeyCode::Enter | KeyCode::Right) => {
                if let Some(selected) = self.list_state.selected() {
                    let entries = self.build_entries_for_reader();
                    if entries.is_empty() {
                        return Ok(AppScreenEvent::None);
                    }
                    let idx = selected.min(entries.len().saturating_sub(1));
                    return Ok(AppScreenEvent::ChangeState(Box::new(ReaderScreen::new(
                        self.library.clone(),
                        entries,
                        idx,
                    ))));
                }
                Ok(AppScreenEvent::None)
            }
            (_, KeyCode::Char('d')) => {
                if let Err(e) = self.remove_selected() {
                    tracing::error!("Failed to remove entry: {:?}", e);
                }
                Ok(AppScreenEvent::None)
            }
            (_, KeyCode::Char('?')) => Ok(AppScreenEvent::OpenDialog(Box::new(HelpDialog::new(
                self.get_full_instructions(),
            )))),
            _ => Ok(AppScreenEvent::None),
        }
    }

    fn get_title(&self) -> String {
        String::from("Read Later")
    }

    fn get_instructions(&self) -> String {
        String::from("?: Help | j/k/↓/↑: move | Enter: read | o: open | d: delete | Esc/q: back")
    }

    fn get_work_status(&self) -> AppWorkStatus {
        AppWorkStatus::None
    }

    fn get_full_instructions(&self) -> String {
        String::from(
            "j/k/↓/↑: move selection\ng/G/Home/End: beginning and end of the list\no: open link externally\nd: delete from read later\n\nEsc/q: back to main screen",
        )
    }
}
