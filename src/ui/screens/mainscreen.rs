use std::{cell::RefCell, rc::Rc};

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
        feed::feedentry::FeedEntry,
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
    library: Rc<RefCell<FeedLibrary>>,
    feedtreestate: FeedTreeState,
    feedentrystate: FeedEntryState,
    inputstate: MainInputState,
}

impl MainScreen {
    pub fn new(library: Rc<RefCell<FeedLibrary>>) -> Self {
        Self {
            library,
            feedtreestate: FeedTreeState::new(),
            feedentrystate: FeedEntryState::new(),
            inputstate: MainInputState::Menu,
        }
    }

    fn set_all_read(&self) {
        let entries = match self.feedtreestate.get_selected() {
            Some(FeedItemInfo::Category(t)) => {
                match self.library.borrow().get_feed_entries_by_category(t) {
                    Ok(entries) => entries,
                    Err(e) => {
                        error!("Error getting feed entries by category: {:?}", e);
                        vec![]
                    }
                }
            }
            Some(FeedItemInfo::Item(_, _, s)) => {
                match self.library.borrow().get_feed_entries_by_item_slug(s) {
                    Ok(entries) => entries,
                    Err(e) => {
                        error!("Error getting feed entries by item slug: {:?}", e);
                        vec![]
                    }
                }
            }
            Some(FeedItemInfo::ReadLater) => {
                match self.library.borrow_mut().get_read_later_feed_entries() {
                    Ok(entries) => entries,
                    Err(e) => {
                        error!("Error getting Read Later entries: {:?}", e);
                        vec![]
                    }
                }
            }
            _ => vec![],
        };

        for entry in entries.iter() {
            self.library.borrow_mut().data.set_entry_seen(entry);
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

    fn toggle_read_later(&mut self, entry: &FeedEntry) {
        let file_path = entry.filepath.to_str().unwrap_or_default();

        if self.library.borrow_mut().is_in_read_later(file_path) {
            if let Err(e) = self.library.borrow_mut().remove_from_read_later(file_path) {
                error!("Failed to remove from read later: {:?}", e);
            }
        } else if let Err(e) = self.library.borrow_mut().add_to_read_later(entry) {
            error!("Failed to add entry to read later: {:?}", e);
        }
    }

    fn increase_tree_width(&mut self) -> color_eyre::Result<()> {
        let mut l = self.library.borrow_mut();
        l.settings.appearance.main_screen_tree_width = l
            .settings
            .appearance
            .main_screen_tree_width
            .saturating_add(2)
            .min(100);
        l.settings.appearance.save()
    }

    fn decrease_tree_width(&mut self) -> color_eyre::Result<()> {
        let mut l = self.library.borrow_mut();
        l.settings.appearance.main_screen_tree_width = l
            .settings
            .appearance
            .main_screen_tree_width
            .saturating_sub(2)
            .min(100);
        l.settings.appearance.save()
    }
}

impl AppScreen for MainScreen {
    fn start(&mut self) {
        self.library.borrow_mut().start_updater();
    }

    fn quit(&mut self) {}

    fn pause(&mut self) {}

    fn unpause(&mut self) {}

    fn render(&mut self, frame: &mut ratatui::Frame, area: Rect) {
        self.library.borrow_mut().update();

        let theme = {
            let library = self.library.borrow();
            library.settings.get_theme().unwrap().clone()
        };

        let treewidth = self
            .library
            .borrow()
            .settings
            .appearance
            .main_screen_tree_width;

        let chunks = Layout::horizontal([
            Constraint::Min(treewidth),
            Constraint::Percentage(85),
            Constraint::Length(1),
        ])
        .split(area);

        // Feed tree
        self.feedtreestate.update(&mut self.library.borrow_mut());

        let (treestyle, treeselectionstyle) = if self.inputstate == MainInputState::Menu {
            (
                Block::default()
                    .style(Style::default().fg(Color::from_u32(theme.base[5])))
                    .bg(Color::from_u32(theme.base[1]))
                    .padding(Padding::new(2, 2, 2, 2)),
                Style::default()
                    .fg(Color::from_u32(theme.base[0xf]))
                    .bg(Color::from_u32(theme.base[0x8])),
            )
        } else {
            (
                Block::default()
                    .style(Style::default().fg(Color::from_u32(theme.base[3])))
                    .bg(Color::from_u32(theme.base[1]))
                    .padding(Padding::new(2, 2, 2, 2)),
                Style::default()
                    .fg(Color::from_u32(theme.base[5]))
                    .bg(Color::from_u32(theme.base[2])),
            )
        };

        let treelist = List::new(self.feedtreestate.get_items(&mut self.library.borrow_mut()))
            .block(treestyle)
            .highlight_style(treeselectionstyle);

        let mut treestate = self.feedtreestate.listatate.clone();
        frame.render_stateful_widget(treelist, chunks[0], &mut treestate);

        // The feed entries
        self.feedentrystate.library = Some(self.library.clone());
        self.feedentrystate
            .update(&mut self.library.borrow_mut(), &self.feedtreestate);

        let mut entryliststate = self.feedentrystate.listatate.clone();

        let entryselectionstyle = if self.inputstate == MainInputState::Content {
            Style::default()
                .fg(Color::from_u32(theme.base[0xf]))
                .bg(Color::from_u32(theme.base[0x8]))
        } else {
            Style::default().bg(Color::from_u32(theme.base[2]))
        };

        let list_widget = List::new(self.feedentrystate.get_items())
            .block(
                Block::default()
                    .style(Style::default().bg(Color::from_u32(theme.base[0])))
                    .padding(Padding::new(2, 2, 1, 1)),
            )
            .highlight_style(entryselectionstyle);

        frame.render_stateful_widget(list_widget, chunks[1], &mut entryliststate);

        // Scrollbar
        let mut scrollbarstate = ScrollbarState::new(self.feedentrystate.scroll_max())
            .position(self.feedentrystate.scroll());
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight).style(
            Style::new()
                .fg(Color::from_u32(theme.base[3]))
                .bg(Color::from_u32(theme.base[1])),
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
                (_, KeyCode::Char('>')) => {
                    self.increase_tree_width()?;
                    Ok(AppScreenEvent::None)
                }
                (_, KeyCode::Char('<')) => {
                    self.decrease_tree_width()?;
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
                (_, KeyCode::Left | KeyCode::Char('h')) => {
                    self.inputstate = MainInputState::Menu;
                    Ok(AppScreenEvent::None)
                }
                (_, KeyCode::Enter) => {
                    if let Some(entry) = self.feedentrystate.get_selected() {
                        self.library.borrow_mut().data.set_entry_seen(&entry);
                        self.feedentrystate.set_current_read();

                        Ok(AppScreenEvent::ChangeState(Box::new(ReaderScreen::new(
                            self.library.clone(),
                            self.feedentrystate.entries.clone(),
                            self.feedentrystate.listatate.selected().unwrap_or(0),
                        ))))
                    } else {
                        Ok(AppScreenEvent::None)
                    }
                }
                (_, KeyCode::Char('r')) => {
                    if let Some(entry) = self.feedentrystate.get_selected() {
                        self.library.borrow_mut().data.toggle_entry_seen(&entry);
                    }
                    Ok(AppScreenEvent::None)
                }
                (_, KeyCode::Char('R')) => {
                    self.set_all_read();
                    Ok(AppScreenEvent::None)
                }
                (_, KeyCode::Char('>')) => {
                    self.increase_tree_width()?;
                    Ok(AppScreenEvent::None)
                }
                (_, KeyCode::Char('<')) => {
                    self.decrease_tree_width()?;
                    Ok(AppScreenEvent::None)
                }
                (_, KeyCode::Char('o')) => {
                    if let Some(entry) = self.feedentrystate.get_selected() {
                        self.library.borrow_mut().data.set_entry_seen(&entry);
                        self.open_external_url(&entry.url)
                    } else {
                        Ok(AppScreenEvent::None)
                    }
                }
                (_, KeyCode::Char('L')) => {
                    if let Some(entry) = self.feedentrystate.get_selected() {
                        self.toggle_read_later(&entry);
                    }

                    Ok(AppScreenEvent::None)
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
            String::from(
                "?: Help | j/k/↓/↑: move | o: open | L: add/remove read later | Enter: read | Esc: back",
            )
        }
    }

    fn get_work_status(&self) -> AppWorkStatus {
        self.library.borrow().get_update_status()
    }

    fn get_full_instructions(&self) -> String {
        String::from(
            "j/k/↓/↑: move selection\ng/G/Home/End: beginning and end of the list\n</>: change reader width\n\no: open link externally\nL: add/remove read later\nEnter: select category or read entry\n\nr: toggle item read state\nR: mark all of the items as read\n\nEsc/q: back from entries or quit",
        )
    }
}
