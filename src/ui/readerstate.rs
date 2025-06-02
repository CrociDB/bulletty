use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    layout::{Constraint, Layout}, widgets::{Block, Borders}
};

use crate::{library::feedlibrary::FeedLibrary, ui::{appstate::AppState, feedtree::FeedTree}};

pub struct ReaderState {
    running: bool,
    library: FeedLibrary,
}

impl ReaderState {
    pub fn new() -> ReaderState {
        ReaderState { running: true, library: FeedLibrary::new() }
    }
}

impl AppState for ReaderState {
    fn start(&mut self) {}

    fn render(&mut self, frame: &mut ratatui::Frame) {
        let layout = Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)]);
        let chunks = layout.split(frame.area());

        let feedtree = FeedTree::new(&self.library);
        frame.render_widget(feedtree, chunks[0]);

        let main_panel = Block::default().title("Main Panel").borders(Borders::ALL);
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
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.running = false,
            _ => {}
        }
    }

    fn quit(&mut self) {

    }

    fn running(&self) -> bool {
        self.running
    }
}
