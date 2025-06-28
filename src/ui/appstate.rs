use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

pub trait AppState {
    fn _start(&mut self);
    fn render(&mut self, frame: &mut Frame, area: Rect);
    fn handle_events(&mut self) -> Result<()>;
    fn handle_keypress(&mut self, key: KeyEvent);
    fn _quit(&mut self);
    fn running(&self) -> bool;
}
