use color_eyre::{Result, eyre};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Paragraph},
};

use crate::ui::appstate::{AppState, AppStateEvent};

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

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.running {
            if let Some(state) = self.current_state.as_mut() {
                terminal.draw(|frame| {
                    let mainlayout =
                        Layout::vertical([Constraint::Percentage(99), Constraint::Min(3)])
                            .margin(1)
                            .split(frame.area());

                    state.render(frame, mainlayout[0]);

                    // Bottom status line
                    let statusline = Layout::horizontal([
                        Constraint::Min(20),
                        Constraint::Percentage(85),
                        Constraint::Min(40),
                    ])
                    .margin(1)
                    .split(mainlayout[1]);

                    let background =
                        Block::default().style(Style::default().bg(Color::from_u32(0x182226)));
                    frame.render_widget(background, mainlayout[1]);

                    let status_text = Paragraph::new("\u{f0fb1} bulletty");
                    frame.render_widget(status_text, statusline[0]);
                })?;

                match state.handle_events()? {
                    AppStateEvent::None => {}
                    AppStateEvent::ChangeState(app_state) => {
                        // Change state...
                    }
                    AppStateEvent::ExitApp => {
                        self.running = false;
                    }
                    AppStateEvent::ExitState => {
                        // Go back to previous state
                    }
                }
            } else {
                self.running = false;
                return Err(eyre::eyre!("No current AppState"));
            }
        }

        Ok(())
    }
}
