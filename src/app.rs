use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use color_eyre::{Result, eyre};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Flex, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Clear, Gauge, Paragraph},
};

use crate::{
    core::{
        library::feedlibrary::FeedLibrary,
        ui::{
            appscreen::{AppScreen, AppScreenEvent},
            dialog::Dialog,
        },
    },
    ui::screens::mainscreen::MainScreen,
};

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
    library: Rc<RefCell<FeedLibrary>>,
    current_state: Option<Box<dyn AppScreen>>,
    states_queue: VecDeque<Box<dyn AppScreen>>,
    dialog_queue: VecDeque<Box<dyn Dialog>>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            library: Rc::new(RefCell::new(FeedLibrary::new())),

            running: true,
            current_state: None,
            states_queue: VecDeque::<Box<dyn AppScreen>>::new(),
            dialog_queue: VecDeque::<Box<dyn Dialog>>::new(),
        }
    }

    pub fn init(&mut self, mut state: Box<dyn AppScreen>) {
        state.start();
        self.current_state = Some(state);
    }

    pub fn initmain(&mut self) {
        self.init(Box::new(MainScreen::new(self.library.clone())));
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
                        Constraint::Max(90),
                    ])
                    .margin(1)
                    .split(mainlayout[1]);

                    let background =
                        Block::default().style(Style::default().bg(Color::from_u32(0x1d1c1c)));
                    frame.render_widget(background, mainlayout[1]);

                    let title = if let Some(dialog) = self.dialog_queue.front() {
                        dialog.as_screen().get_title()
                    } else {
                        state.get_title()
                    };

                    let status_text = Paragraph::new(format!("\u{f0fb1} bulletty | {title}"));

                    frame.render_widget(status_text, statusline[0]);

                    let instructions = if let Some(dialog) = self.dialog_queue.front() {
                        dialog.as_screen().get_instructions()
                    } else {
                        state.get_instructions()
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
                    if let Some(dialog) = self.dialog_queue.get_mut(0) {
                        let overlay = Block::default().style(
                            Style::default()
                                .bg(Color::from_u32(0x575653))
                                .fg(Color::Reset),
                        );
                        frame.render_widget(overlay, mainlayout[0]);

                        let border = Block::new().style(Style::new().bg(Color::from_u32(0x262626)));
                        let block = Block::new().style(Style::new().bg(Color::from_u32(0x3a3a3a)));

                        let area = popup_area(mainlayout[0], dialog.get_size());
                        let inner_area = area.inner(Margin::new(2, 1));

                        frame.render_widget(Clear, area);
                        frame.render_widget(border, area);
                        frame.render_widget(block, inner_area);

                        dialog.as_screen_mut().render(frame, inner_area);
                    }
                })?;

                // Checking the dialog or the state events
                let event = if let Some(dialog) = self.dialog_queue.get_mut(0) {
                    dialog.as_screen_mut().handle_events()?
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
            let status = state.get_work_status();
            if !status.is_none() {
                return status;
            }
        }

        self.states_queue
            .iter()
            .map(|state| state.get_work_status())
            .find(|state| !state.is_none())
            .unwrap_or(AppWorkStatus::None)
    }

    fn open_dialog(&mut self, mut dialog_state: Box<dyn Dialog>) {
        if let Some(state) = self.current_state.as_mut() {
            state.pause();
        }

        dialog_state.as_screen_mut().start();
        self.dialog_queue.push_back(dialog_state);
    }

    fn close_current_dialog(&mut self) {
        if let Some(mut state) = self.dialog_queue.pop_back() {
            state.as_screen_mut().quit();
        }
    }
}

fn popup_area(area: Rect, size: Rect) -> Rect {
    let mut vertical = Layout::vertical([Constraint::Percentage(size.height)]).flex(Flex::Center);
    let mut horizontal =
        Layout::horizontal([Constraint::Percentage(size.width)]).flex(Flex::Center);

    if size.x + size.y > 0 {
        vertical = Layout::vertical([Constraint::Length(size.y)]).flex(Flex::Center);
        horizontal = Layout::horizontal([Constraint::Length(size.x)]).flex(Flex::Center);
    }

    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);

    area
}
