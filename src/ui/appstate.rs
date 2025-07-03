use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};

use crate::app::AppWorkStatus;

pub enum AppStateEvent {
    None,
    ChangeState(Box<dyn AppState>),
    ExitState,
    ExitApp,
}

pub trait AppState {
    fn start(&mut self);
    fn quit(&mut self);

    fn pause(&mut self);
    fn unpause(&mut self);

    fn render(&mut self, frame: &mut Frame, area: Rect);
    fn handle_events(&mut self) -> Result<AppStateEvent>;
    fn handle_keypress(&mut self, key: KeyEvent) -> Result<AppStateEvent>;

    fn get_state_work_status(&self) -> AppWorkStatus;
    fn get_state_name(&self) -> String;
    fn get_state_instructions(&self) -> String;
}
