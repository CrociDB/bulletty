use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, List, Padding, Scrollbar, ScrollbarOrientation, ScrollbarState},
};
use tracing::error;

use crate::{
    app::AppWorkStatus,
    core::{
        library::feedlibrary::FeedLibrary,
        ui::appscreen::{AppScreen, AppScreenEvent},
    },
    ui::{
        screens::{readerscreen::ReaderScreen, urldialog::UrlDialog},
        states::{
            feedentrystate::FeedEntryState,
            feedtreestate::{FeedItemInfo, FeedTreeState},
        },
    },
};

use super::helpdialog::HelpDialog;

#[derive(PartialEq, Eq)]
enum MainInputState {
    Menu,
    Content,
}

pub struct MainScreen {
    library: FeedLibrary,
    feedtreestate: FeedTreeState,
    feedentrystate: FeedEntryState,
    inputstate: MainInputState,
}

impl Default for MainScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl MainScreen {
    pub fn new() -> Self {
        Self {
            library: FeedLibrary::new(),
            feedtreestate: FeedTreeState::new(),
            feedentrystate: FeedEntryState::new(),
            inputstate: MainInputState::Menu,
        }
    }

    fn set_all_read(&self) {
        let entries = match self.feedtreestate.get_selected() {
            Some(FeedItemInfo::Category(t)) => self.library.get_feed_entries_by_category(t),
            Some(FeedItemInfo::Item(_, _, s)) => self.library.get_feed_entries_by_item_slug(s),
            None => vec![],
        };

        for entry in entries.iter() {
            self.library.data.set_entry_seen(entry);
        }
    }

    fn open_external_url(&self, url: &str) -> Result<AppScreenEvent> {
        match open::that(url) {
            Ok(_) => Ok(AppScreenEvent::None),
            Err(_) => {
                error!("Couldn't invoke system browser");
                Ok(AppScreenEvent::OpenDialog(Box::new(UrlDialog::new(
                    url.to_string(),
                ))))
            }
        }
    }
}

impl AppScreen for MainScreen {
    fn start(&mut self) {
        self.library.start_updater();
    }

    fn quit(&mut self) {}

    fn pause(&mut self) {}

    fn unpause(&mut self) {}

    fn render(&mut self, frame: &mut ratatui::Frame, area: Rect) {
        self.library.update();

        let chunks = Layout::horizontal([
            Constraint::Min(30),
            Constraint::Percentage(85),
            Constraint::Min(1),
        ])
        .split(area);

        // Feed tree
        self.feedtreestate.update(&self.library);

        let (treestyle, treeselectionstyle) = if self.inputstate == MainInputState::Menu {
            (
                Block::default()
                    .style(Style::default().bg(Color::from_u32(0x262626)))
                    .padding(Padding::new(2, 2, 2, 2)),
                Style::default().bg(Color::from_u32(0x514537)),
            )
        } else {
            (
                Block::default()
                    .style(Style::default().bg(Color::from_u32(0x262626)))
                    .dim()
                    .padding(Padding::new(2, 2, 2, 2)),
                Style::default().bg(Color::DarkGray),
            )
        };

        let treelist = List::new(self.feedtreestate.get_items(&self.library))
            .block(treestyle)
            .highlight_style(treeselectionstyle);

        let mut treestate = self.feedtreestate.listatate.clone();
        frame.render_stateful_widget(treelist, chunks[0], &mut treestate);

        // The feed entries
        self.feedentrystate
            .update(&self.library, &self.feedtreestate);

        let mut entryliststate = self.feedentrystate.listatate.clone();

        let entryselectionstyle = if self.inputstate == MainInputState::Content {
            Style::default().bg(Color::from_u32(0x514537))
        } else {
            Style::default().bg(Color::DarkGray)
        };

        let list_widget = List::new(self.feedentrystate.get_items())
            .block(
                Block::default()
                    .style(Style::default().bg(Color::from_u32(0x3a3a3a)))
                    .padding(Padding::new(2, 2, 1, 1)),
            )
            .highlight_style(entryselectionstyle);

        frame.render_stateful_widget(list_widget, chunks[1], &mut entryliststate);

        // Scrollbar
        let mut scrollbarstate = ScrollbarState::new(self.feedentrystate.scroll_max())
            .position(self.feedentrystate.scroll());
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight).style(
            Style::new()
                .fg(Color::from_u32(0x555555))
                .bg(Color::from_u32(0x3a3a3a)),
        );
        frame.render_stateful_widget(scrollbar, chunks[2], &mut scrollbarstate);
    }

    fn handle_events(&mut self) -> Result<AppScreenEvent> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.handle_keypress(key),
            Event::Mouse(_) => Ok(AppScreenEvent::None),
            Event::Resize(_, _) => Ok(AppScreenEvent::None),
            _ => Ok(AppScreenEvent::None),
        }
    }

    fn handle_keypress(&mut self, key: crossterm::event::KeyEvent) -> Result<AppScreenEvent> {
        match self.inputstate {
            MainInputState::Menu => match (key.modifiers, key.code) {
                (_, KeyCode::Esc | KeyCode::Char('q'))
                | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                    Ok(AppScreenEvent::ExitApp)
                }
                (_, KeyCode::Down | KeyCode::Char('j')) => {
                    self.feedtreestate.select_next();
                    Ok(AppScreenEvent::None)
                }
                (_, KeyCode::Up | KeyCode::Char('k')) => {
                    self.feedtreestate.select_previous();
                    Ok(AppScreenEvent::None)
                }
                (_, KeyCode::Home | KeyCode::Char('g')) => {
                    self.feedtreestate.select_first();
                    Ok(AppScreenEvent::None)
                }
                (_, KeyCode::End | KeyCode::Char('G')) => {
                    self.feedtreestate.select_last();
                    Ok(AppScreenEvent::None)
                }
                (_, KeyCode::Right | KeyCode::Enter | KeyCode::Tab | KeyCode::Char('l')) => {
                    self.inputstate = MainInputState::Content;
                    Ok(AppScreenEvent::None)
                }
                (_, KeyCode::Char('R')) => {
                    self.set_all_read();
                    Ok(AppScreenEvent::None)
                }
                (_, KeyCode::Char('?')) => Ok(AppScreenEvent::OpenDialog(Box::new(
                    HelpDialog::new(self.get_full_instructions()),
                ))),
                _ => Ok(AppScreenEvent::None),
            },
            MainInputState::Content => match (key.modifiers, key.code) {
                (_, KeyCode::Char('q'))
                | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                    Ok(AppScreenEvent::ExitApp)
                }
                (_, KeyCode::Down | KeyCode::Char('j')) => {
                    self.feedentrystate.select_next();
                    Ok(AppScreenEvent::None)
                }
                (_, KeyCode::Up | KeyCode::Char('k')) => {
                    self.feedentrystate.select_previous();
                    Ok(AppScreenEvent::None)
                }
                (_, KeyCode::Home | KeyCode::Char('g')) => {
                    self.feedentrystate.select_first();
                    Ok(AppScreenEvent::None)
                }
                (_, KeyCode::End | KeyCode::Char('G')) => {
                    self.feedentrystate.select_last();
                    Ok(AppScreenEvent::None)
                }
                (_, KeyCode::Esc) => {
                    self.inputstate = MainInputState::Menu;
                    Ok(AppScreenEvent::None)
                }
                (_, KeyCode::Right | KeyCode::Char('h')) => {
                    self.inputstate = MainInputState::Menu;
                    Ok(AppScreenEvent::None)
                }
                (_, KeyCode::Enter) => {
                    if let Some(entry) = self.feedentrystate.get_selected() {
                        self.library.data.set_entry_seen(&entry);
                        self.feedentrystate.set_current_read();

                        Ok(AppScreenEvent::ChangeState(Box::new(ReaderScreen::new(
                            entry,
                        ))))
                    } else {
                        Ok(AppScreenEvent::None)
                    }
                }
                (_, KeyCode::Char('r')) => {
                    if let Some(entry) = self.feedentrystate.get_selected() {
                        self.library.data.toggle_entry_seen(&entry);
                    }
                    Ok(AppScreenEvent::None)
                }
                (_, KeyCode::Char('R')) => {
                    self.set_all_read();
                    Ok(AppScreenEvent::None)
                }
                (_, KeyCode::Char('o')) => {
                    if let Some(entry) = self.feedentrystate.get_selected() {
                        self.library.data.set_entry_seen(&entry);
                        self.open_external_url(&entry.url)
                    } else {
                        Ok(AppScreenEvent::None)
                    }
                }
                (_, KeyCode::Char('?')) => Ok(AppScreenEvent::OpenDialog(Box::new(
                    HelpDialog::new(self.get_full_instructions()),
                ))),
                _ => Ok(AppScreenEvent::None),
            },
        }
    }

    fn get_title(&self) -> String {
        String::from("Main")
    }

    fn get_instructions(&self) -> String {
        if self.inputstate == MainInputState::Menu {
            String::from("?: Help | j/k/↓/↑: move | Enter: select | Esc: quit")
        } else {
            String::from("?: Help | j/k/↓/↑: move | o: open | Enter: read | Esc: back")
        }
    }

    fn get_work_status(&self) -> AppWorkStatus {
        self.library.get_update_status()
    }

    fn get_full_instructions(&self) -> String {
        String::from(
            "j/k/↓/↑: move selection\ng/G/Home/End: beginning and end of the list\no: open link externally\nEnter: select category or read entry\n\nr: toggle item read state\nR: mark all of the items as read\n\nEsc/q: back from entries or quit",
        )
    }
}
