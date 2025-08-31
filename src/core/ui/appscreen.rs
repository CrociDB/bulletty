use color_eyre::Result;
use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};

use crate::app::AppWorkStatus;

pub enum AppScreenEvent {
    None,

    ChangeState(Box<dyn AppScreen>),
    ExitState,

    OpenDialog(Box<dyn AppScreen>),
    CloseDialog,

    ExitApp,
}

pub trait AppScreen {
    fn start(&mut self);
    fn quit(&mut self);

    fn pause(&mut self);
    fn unpause(&mut self);

    fn render(&mut self, frame: &mut Frame, area: Rect);
    fn handle_events(&mut self) -> Result<AppScreenEvent>;
    fn handle_keypress(&mut self, key: KeyEvent) -> Result<AppScreenEvent>;

    fn get_state_work_status(&self) -> AppWorkStatus;
    fn get_state_name(&self) -> String;
    fn get_state_instructions(&self) -> String;
}
