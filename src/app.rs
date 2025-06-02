use color_eyre::{eyre, Result};
use ratatui::DefaultTerminal;

use crate::appstate::AppState;

// #[derive(Debug)]
pub struct App {
    running: bool,
    current_state: Option<Box<dyn AppState>>,
}

impl App {
    pub fn new() -> Self {
        App {
            running: true,
            current_state: None,
        }
    }

    pub fn init(&mut self, state: Box<dyn AppState>) {
        self.current_state = Some(state);
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.running {
            if let Some(state) = self.current_state.as_mut() {
                terminal.draw(|frame| state.render(frame))?;
                state.handle_events()?;
                self.running = state.running();
            } else {
                self.running = false;
                return Err(eyre::eyre!("No current AppState"));
            }
        }

        Ok(())
    }
}
