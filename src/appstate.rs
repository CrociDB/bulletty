use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::Frame;

pub trait AppState {
    fn start(&mut self);
    fn render(&mut self, frame: &mut Frame);
    fn handle_events(&mut self) -> Result<()>;
    fn handle_keypress(&mut self, key: KeyEvent);
    fn quit(&mut self);
    fn running(&self) -> bool;
}
