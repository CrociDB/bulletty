use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::{Alignment, Constraint, Layout, Margin, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Paragraph, Wrap};

use crate::app::AppWorkStatus;

use crate::core::ui::appscreen::{AppScreen, AppScreenEvent};
use crate::core::ui::dialog::Dialog;

pub struct HelpDialog {
    help_string: String,
}

impl HelpDialog {
    pub fn new(help_string: String) -> HelpDialog {
        HelpDialog { help_string }
    }
}

impl Dialog for HelpDialog {
    fn get_size(&self) -> ratatui::prelude::Rect {
        Rect::new(0, 0, 30, 40)
    }

    fn as_screen(&self) -> &dyn AppScreen {
        self
    }

    fn as_screen_mut(&mut self) -> &mut dyn AppScreen {
        self
    }
}

impl AppScreen for HelpDialog {
    fn start(&mut self) {}

    fn quit(&mut self) {}

    fn pause(&mut self) {}

    fn unpause(&mut self) {}

    fn render(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        let contentlayout = Layout::vertical([Constraint::Length(2), Constraint::Fill(1)])
            .split(area.inner(Margin::new(2, 1)));

        let title = Paragraph::new(self.get_title())
            .style(Style::new().fg(Color::LightRed))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        let content = Paragraph::new(self.help_string.to_string())
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(title, contentlayout[0]);
        frame.render_widget(content, contentlayout[1]);
    }

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

    fn get_work_status(&self) -> AppWorkStatus {
        AppWorkStatus::None
    }

    fn get_title(&self) -> String {
        String::from("Help")
    }

    fn get_instructions(&self) -> String {
        String::from("Esc/q: close help")
    }

    fn get_full_instructions(&self) -> String {
        self.get_instructions()
    }
}
