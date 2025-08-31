use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, List, Padding},
};

use crate::{
    app::AppWorkStatus,
    library::feedlibrary::FeedLibrary,
    ui::{
        appstate::{AppState, AppStateEvent},
        feedentrystate::FeedEntryState,
        feedtreestate::FeedTreeState,
        readerstate::ReaderState,
    },
};

use super::helpdialog::HelpDialog;

#[derive(PartialEq, Eq)]
enum MainInputState {
    Menu,
    Content,
}

pub struct MainState {
    library: FeedLibrary,
    feedtreestate: FeedTreeState,
    feedentrystate: FeedEntryState,
    inputstate: MainInputState,
}

impl MainState {
    pub fn new() -> MainState {
        MainState {
            library: FeedLibrary::new(),
            feedtreestate: FeedTreeState::new(),
            feedentrystate: FeedEntryState::new(),
            inputstate: MainInputState::Menu,
        }
    }
}

impl AppState for MainState {
    fn start(&mut self) {
        self.library.start_updater();
    }

    fn quit(&mut self) {}

    fn pause(&mut self) {}

    fn unpause(&mut self) {}

    fn render(&mut self, frame: &mut ratatui::Frame, area: Rect) {
        self.library.update();

        let chunks =
            Layout::horizontal([Constraint::Min(30), Constraint::Percentage(85)]).split(area);

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
    }

    fn handle_events(&mut self) -> Result<AppStateEvent> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.handle_keypress(key),
            Event::Mouse(_) => Ok(AppStateEvent::None),
            Event::Resize(_, _) => Ok(AppStateEvent::None),
            _ => Ok(AppStateEvent::None),
        }
    }

    fn handle_keypress(&mut self, key: crossterm::event::KeyEvent) -> Result<AppStateEvent> {
        match self.inputstate {
            MainInputState::Menu => match (key.modifiers, key.code) {
                (_, KeyCode::Esc | KeyCode::Char('q'))
                | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                    Ok(AppStateEvent::ExitApp)
                }
                (_, KeyCode::Down | KeyCode::Char('j')) => {
                    self.feedtreestate.select_next();
                    Ok(AppStateEvent::None)
                }
                (_, KeyCode::Up | KeyCode::Char('k')) => {
                    self.feedtreestate.select_previous();
                    Ok(AppStateEvent::None)
                }
                (_, KeyCode::Right | KeyCode::Enter | KeyCode::Tab | KeyCode::Char('l')) => {
                    self.inputstate = MainInputState::Content;
                    Ok(AppStateEvent::None)
                }
                (_, KeyCode::Char('?')) => Ok(AppStateEvent::OpenDialog(Box::new(
                    HelpDialog::new(self.get_state_instructions()),
                ))),
                _ => Ok(AppStateEvent::None),
            },
            MainInputState::Content => match (key.modifiers, key.code) {
                (_, KeyCode::Char('q'))
                | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                    Ok(AppStateEvent::ExitApp)
                }
                (_, KeyCode::Down | KeyCode::Char('j')) => {
                    self.feedentrystate.select_next();
                    Ok(AppStateEvent::None)
                }
                (_, KeyCode::Up | KeyCode::Char('k')) => {
                    self.feedentrystate.select_previous();
                    Ok(AppStateEvent::None)
                }
                (_, KeyCode::Esc) => {
                    self.inputstate = MainInputState::Menu;
                    Ok(AppStateEvent::None)
                }
                (_, KeyCode::Right | KeyCode::Char('h')) => {
                    self.inputstate = MainInputState::Menu;
                    Ok(AppStateEvent::None)
                }
                (_, KeyCode::Enter) => {
                    if let Some(entry) = self.feedentrystate.get_selected() {
                        self.library.data.set_entry_seen(&entry);
                        self.feedentrystate.set_current_read();

                        Ok(AppStateEvent::ChangeState(Box::new(ReaderState::new(
                            entry,
                        ))))
                    } else {
                        Ok(AppStateEvent::None)
                    }
                }
                (_, KeyCode::Char('?')) => Ok(AppStateEvent::OpenDialog(Box::new(
                    HelpDialog::new(self.get_state_instructions()),
                ))),
                _ => Ok(AppStateEvent::None),
            },
        }
    }

    fn get_state_name(&self) -> String {
        String::from("Main")
    }

    fn get_state_instructions(&self) -> String {
        if self.inputstate == MainInputState::Menu {
            String::from("j/k/↓/↑: move selection | Enter: select | Esc/q: quit")
        } else {
            String::from("j/k/↓/↑: move selection | Enter: read entry | Esc/q: back")
        }
    }

    fn get_state_work_status(&self) -> AppWorkStatus {
        self.library.get_update_status()
    }
}
