use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
};

use crate::{
    library::feedlibrary::FeedLibrary,
    ui::{appstate::AppState, feedtree::FeedTree},
};

#[derive(PartialEq, Eq)]
enum ReaderInputState {
    Menu,
    Content,
}

pub struct ReaderState {
    running: bool,
    library: FeedLibrary,
    inputstate: ReaderInputState,
}

impl ReaderState {
    pub fn new() -> ReaderState {
        ReaderState {
            running: true,
            library: FeedLibrary::new(),
            inputstate: ReaderInputState::Menu,
        }
    }
}

impl AppState for ReaderState {
    fn _start(&mut self) {}

    fn render(&mut self, frame: &mut ratatui::Frame) {
        let chunks = Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)])
            .margin(1)
            .split(frame.area());

        let disabled_style = Style::default().fg(Color::Gray).add_modifier(Modifier::DIM);

        let mut feedtree = FeedTree::new();
        feedtree.enabled = self.inputstate == ReaderInputState::Menu;
        feedtree.set_list_data(&(self.library));
        frame.render_widget(feedtree, chunks[0]);

        let mut main_panel = Block::default().title("Main Panel").borders(Borders::ALL);
        if self.inputstate == ReaderInputState::Menu {
            main_panel = main_panel.style(disabled_style);
        }
        frame.render_widget(main_panel, chunks[1]);
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
                    self.library.selection_down();
                }
                (_, KeyCode::Up | KeyCode::Char('k')) => {
                    self.library.selection_up();
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
