use std::time::Duration;

use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::{Alignment, Constraint, Layout, Margin, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Paragraph, Wrap};

use crate::app::AppWorkStatus;

use crate::core::ui::appscreen::{AppScreen, AppScreenEvent};
use crate::core::ui::dialog::Dialog;

pub struct UrlDialog {
    url: String,
}

impl UrlDialog {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

impl Dialog for UrlDialog {
    fn get_size(&self) -> ratatui::prelude::Rect {
        Rect::new((self.url.len() as u16) + 10, 7, 0, 0)
    }

    fn as_screen(&self) -> &dyn AppScreen {
        self
    }

    fn as_screen_mut(&mut self) -> &mut dyn AppScreen {
        self
    }
}

impl AppScreen for UrlDialog {
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

        let content = Paragraph::new(self.url.to_string())
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(title, contentlayout[0]);
        frame.render_widget(content, contentlayout[1]);
    }

    fn handle_events(&mut self) -> Result<AppScreenEvent> {
        if !event::poll(Duration::from_millis(100))? {
            return Ok(AppScreenEvent::None);
        }

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
        String::from("Couldn't open browser automatically")
    }

    fn get_instructions(&self) -> String {
        String::from("Esc/q: close url")
    }

    fn get_full_instructions(&self) -> String {
        self.get_instructions()
    }
}
