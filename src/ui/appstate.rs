use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};

pub enum AppStateEvent {
    None,
    ChangeState(Box<dyn AppState>),
    ExitState,
    ExitApp,
}

pub trait AppState {
    fn _start(&mut self);
    fn render(&mut self, frame: &mut Frame, area: Rect);
    fn handle_events(&mut self) -> Result<AppStateEvent>;
    fn handle_keypress(&mut self, key: KeyEvent) -> Result<AppStateEvent>;
    fn _quit(&mut self);
}
