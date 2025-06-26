use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, List, Padding},
};

use crate::{
    library::feedlibrary::FeedLibrary,
    ui::{appstate::AppState, feedentrystate::FeedEntryState, feedtreestate::FeedTreeState},
};

#[derive(PartialEq, Eq)]
enum ReaderInputState {
    Menu,
    Content,
}

pub struct ReaderState {
    running: bool,
    library: FeedLibrary,
    feedtreestate: FeedTreeState,
    feedentrystate: FeedEntryState,
    inputstate: ReaderInputState,
}

impl ReaderState {
    pub fn new() -> ReaderState {
        ReaderState {
            running: true,
            library: FeedLibrary::new(),
            feedtreestate: FeedTreeState::new(),
            feedentrystate: FeedEntryState::new(),
            inputstate: ReaderInputState::Menu,
        }
    }
}

impl AppState for ReaderState {
    fn _start(&mut self) {}

    fn render(&mut self, frame: &mut ratatui::Frame) {
        let chunks = Layout::horizontal([Constraint::Min(30), Constraint::Percentage(85)])
            .margin(1)
            .split(frame.area());

        // Feed tree
        self.feedtreestate.update(&self.library);

        let block_style = if self.inputstate == ReaderInputState::Menu {
            Block::default()
                .style(Style::default().bg(Color::from_u32(0x262626)))
                .padding(Padding::new(2, 2, 2, 2))
        } else {
            Block::default()
                .style(Style::default().bg(Color::from_u32(0x262626)))
                .dim()
                .padding(Padding::new(2, 2, 2, 2))
        };

        let treelist = List::new(self.feedtreestate.get_items())
            .block(block_style)
            .highlight_style(Style::default().bg(Color::Yellow));

        let mut treestate = self.feedtreestate.listatate.clone();
        frame.render_stateful_widget(treelist, chunks[0], &mut treestate);

        // The feed entries
        self.feedentrystate
            .update(&self.library, &self.feedtreestate);

        let mut entryliststate = self.feedentrystate.listatate.clone();

        let list_widget = List::new(self.feedentrystate.get_items())
            .block(
                Block::default()
                    .style(Style::default().bg(Color::from_u32(0x3a3a3a)))
                    .padding(Padding::new(2, 2, 1, 1)),
            )
            .highlight_style(Style::default().bg(Color::Blue));

        frame.render_stateful_widget(list_widget, chunks[1], &mut entryliststate);
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.handle_keypress(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    fn handle_keypress(&mut self, key: crossterm::event::KeyEvent) {
        match self.inputstate {
            ReaderInputState::Menu => match (key.modifiers, key.code) {
                (_, KeyCode::Esc | KeyCode::Char('q'))
                | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                    self.running = false;
                }
                (_, KeyCode::Down | KeyCode::Char('j')) => {
                    self.feedtreestate.selection_down();
                }
                (_, KeyCode::Up | KeyCode::Char('k')) => {
                    self.feedtreestate.selection_up();
                }
                (_, KeyCode::Right | KeyCode::Enter | KeyCode::Tab | KeyCode::Char('l')) => {
                    self.inputstate = ReaderInputState::Content;
                }
                _ => {}
            },
            ReaderInputState::Content => match (key.modifiers, key.code) {
                (_, KeyCode::Char('q'))
                | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                    self.running = false;
                }
                (_, KeyCode::Down | KeyCode::Char('j')) => {
                    self.feedentrystate.selection_down();
                }
                (_, KeyCode::Up | KeyCode::Char('k')) => {
                    self.feedentrystate.selection_up();
                }
                (_, KeyCode::Esc) => {
                    self.inputstate = ReaderInputState::Menu;
                }
                (_, KeyCode::Right | KeyCode::Char('h')) => {
                    self.inputstate = ReaderInputState::Menu;
                }
                _ => {}
            },
        }
    }

    fn _quit(&mut self) {}

    fn running(&self) -> bool {
        self.running
    }
}
