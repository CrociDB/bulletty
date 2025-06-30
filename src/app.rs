use std::collections::VecDeque;

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
    states_queue: VecDeque<Box<dyn AppState>>,
}

impl App {
    pub fn new() -> Self {
        App {
            running: true,
            current_state: None,
            states_queue: VecDeque::<Box<dyn AppState>>::new(),
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
                        self.change_state(app_state);
                    }
                    AppStateEvent::ExitApp => {
                        self.running = false;
                    }
                    AppStateEvent::ExitState => {
                        self.exit_state();
                    }
                }
            } else {
                self.running = false;
                return Err(eyre::eyre!("No current AppState"));
            }
        }

        Ok(())
    }

    fn change_state(&mut self, mut new_state: Box<dyn AppState>) {
        if let Some(mut state) = self.current_state.take() {
            state.pause();
            self.states_queue.push_back(state);
        }

        new_state.start();
        self.current_state = Some(new_state);
    }

    fn exit_state(&mut self) {
        if let Some(mut state) = self.current_state.take() {
            state.quit();
        }

        if !self.states_queue.is_empty() {
            if let Some(mut state) = self.states_queue.pop_back() {
                state.unpause();
                self.current_state = Some(state);
            } else {
                self.running = false;
            }
        } else {
            self.running = false;
        }
    }
}
