use std::collections::VecDeque;

use color_eyre::{Result, eyre};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Clear, Gauge, Paragraph},
};

use crate::core::ui::appscreen::{AppScreen, AppScreenEvent};

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
    current_state: Option<Box<dyn AppScreen>>,
    states_queue: VecDeque<Box<dyn AppScreen>>,
    dialog_queue: VecDeque<Box<dyn AppScreen>>,
}

impl App {
    pub fn new() -> Self {
        App {
            running: true,
            current_state: None,
            states_queue: VecDeque::<Box<dyn AppScreen>>::new(),
            dialog_queue: VecDeque::<Box<dyn AppScreen>>::new(),
        }
    }

    pub fn init(&mut self, mut state: Box<dyn AppScreen>) {
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

                    let title = if let Some(dialog) = self.dialog_queue.front() {
                        dialog.get_state_name()
                    } else {
                        state.get_state_name()
                    };

                    let status_text = Paragraph::new(format!("\u{f0fb1} bulletty | {title}"));

                    frame.render_widget(status_text, statusline[0]);

                    let instructions = if let Some(dialog) = self.dialog_queue.front() {
                        dialog.get_state_instructions()
                    } else {
                        state.get_state_instructions()
                    };

                    let instructions_text = Paragraph::new(instructions.to_string())
                        .style(Style::default().dim())
                        .alignment(ratatui::layout::Alignment::Right);
                    frame.render_widget(instructions_text, statusline[2]);

                    // work status
                    if let AppWorkStatus::Working(percentage, description) = work_status {
                        let gauge = Gauge::default()
                            .gauge_style(
                                Style::default()
                                    .fg(Color::from_u32(0x81ae80))
                                    .bg(Color::Black),
                            )
                            .percent((percentage * 100.0).round() as u16)
                            .label(&description);
                        frame.render_widget(gauge, statusline[1]);
                    }

                    // After drawing the state, needs to check if there's a dialog
                    if let Some(_dialog) = self.dialog_queue.get_mut(0) {
                        let overlay = Block::default()
                            .style(Style::default().bg(Color::DarkGray).fg(Color::Reset));
                        frame.render_widget(overlay, frame.area());

                        let block = Block::bordered().title("Popup");
                        let area = popup_area(frame.area(), 60, 20);
                        frame.render_widget(Clear, area);
                        frame.render_widget(block, area);
                    }
                })?;

                // Checking the dialog or the state events
                let event = if let Some(dialog) = self.dialog_queue.get_mut(0) {
                    dialog.handle_events()?
                } else {
                    state.handle_events()?
                };

                match event {
                    AppScreenEvent::None => {}

                    AppScreenEvent::ChangeState(app_state) => {
                        self.change_state(app_state);
                    }
                    AppScreenEvent::ExitState => {
                        self.exit_state();
                    }

                    AppScreenEvent::OpenDialog(app_state) => {
                        self.open_dialog(app_state);
                    }
                    AppScreenEvent::CloseDialog => {
                        self.close_current_dialog();
                    }

                    AppScreenEvent::ExitApp => {
                        self.running = false;
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

    fn change_state(&mut self, mut new_state: Box<dyn AppScreen>) {
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

    fn open_dialog(&mut self, mut dialog_state: Box<dyn AppScreen>) {
        if let Some(state) = self.current_state.as_mut() {
            state.pause();
        }

        dialog_state.start();
        self.dialog_queue.push_back(dialog_state);
    }

    fn close_current_dialog(&mut self) {
        if let Some(mut state) = self.dialog_queue.pop_back() {
            state.quit();
        }
    }
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
