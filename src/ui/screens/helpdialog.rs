use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

use crate::app::AppWorkStatus;

use crate::core::ui::appscreen::{AppScreen, AppScreenEvent};

pub struct HelpDialog {
    help_string: String,
}

impl HelpDialog {
    pub fn new(help_string: String) -> HelpDialog {
        HelpDialog { help_string }
    }
}

impl AppScreen for HelpDialog {
    fn start(&mut self) {}

    fn quit(&mut self) {}

    fn pause(&mut self) {}

    fn unpause(&mut self) {}

    fn render(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {}

    fn handle_events(&mut self) -> Result<AppScreenEvent> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.handle_keypress(key),
            Event::Mouse(_) => Ok(AppScreenEvent::None),
            Event::Resize(_, _) => Ok(AppScreenEvent::None),
            _ => Ok(AppScreenEvent::None),
        }
    }

    fn handle_keypress(&mut self, key: KeyEvent) -> Result<AppScreenEvent> {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                Ok(AppScreenEvent::CloseDialog)
            }
            _ => Ok(AppScreenEvent::None),
        }
    }

    fn get_state_work_status(&self) -> AppWorkStatus {
        AppWorkStatus::None
    }

    fn get_state_name(&self) -> String {
        String::from("Help")
    }

    fn get_state_instructions(&self) -> String {
        String::from("Esc/q: close help")
    }
}
