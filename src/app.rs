use std::collections::VecDeque;

use color_eyre::{Result, eyre};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, Gauge, Paragraph},
};

use crate::ui::appstate::{AppState, AppStateEvent};

pub enum AppWorkStatus {
    None,
    Working(f32, String),
}

impl AppWorkStatus {
    pub fn is_none(&self) -> bool {
        matches!(self, AppWorkStatus::None)
    }
}

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

    pub fn init(&mut self, mut state: Box<dyn AppState>) {
        state.start();
        self.current_state = Some(state);
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.running {
            let work_status = self.get_work_status();
            if let Some(state) = self.current_state.as_mut() {
                terminal.draw(|frame| {
                    let mainlayout =
                        Layout::vertical([Constraint::Percentage(99), Constraint::Min(3)])
                            .margin(1)
                            .split(frame.area());

                    state.render(frame, mainlayout[0]);

                    // Bottom status line
                    let statusline = Layout::horizontal([
                        Constraint::Max(25),
                        Constraint::Fill(1),
                        Constraint::Max(60),
                    ])
                    .margin(1)
                    .split(mainlayout[1]);

                    let background =
                        Block::default().style(Style::default().bg(Color::from_u32(0x1d1c1c)));
                    frame.render_widget(background, mainlayout[1]);

                    let status_text =
                        Paragraph::new(format!("\u{f0fb1} bulletty | {}", state.get_state_name()));
                    frame.render_widget(status_text, statusline[0]);

                    let instructions_text = Paragraph::new(state.get_state_instructions().clone())
                        .style(Style::default().dim())
                        .alignment(ratatui::layout::Alignment::Right);
                    frame.render_widget(instructions_text, statusline[2]);
                    // work status
                    if let AppWorkStatus::Working(percentage, description) = work_status {
                        let gauge = Gauge::default()
                            .gauge_style(Style::default().fg(Color::from_u32(0x81ae80)).bg(Color::Black))
                            .percent((percentage * 100.0).round() as u16)
                            .label(&description);
                        frame.render_widget(gauge, statusline[1]);
                    }
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

        if let Some(state) = self.current_state.as_mut() {
            state.quit();
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

    fn get_work_status(&self) -> AppWorkStatus {
        if let Some(state) = self.current_state.as_ref() {
            let status = state.get_state_work_status();
            if !status.is_none() {
                return status;
            }
        }

        self.states_queue
            .iter()
            .map(|state| state.get_state_work_status())
            .find(|state| !state.is_none())
            .unwrap_or(AppWorkStatus::None)
    }
}
