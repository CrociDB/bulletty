use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::Alignment;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Padding, Paragraph, Wrap};

use crate::{
    feed::feedentry::FeedEntry,
    ui::appstate::{AppState, AppStateEvent},
};

pub struct ReaderState {
    feedentry: FeedEntry,
}

impl ReaderState {
    pub fn new(entry: FeedEntry) -> ReaderState {
        ReaderState { feedentry: entry }
    }
}

impl AppState for ReaderState {
    fn start(&mut self) {}

    fn render(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        let block = Block::default()
            .style(Style::default().bg(Color::from_u32(0x262626)))
            .padding(Padding::new(3, 3, 3, 3));


        let paragraph = Paragraph::new(self.feedentry.title.clone())
            .block(block)
            .style(Style::new().fg(Color::White))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }

    fn handle_events(&mut self) -> Result<AppStateEvent> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.handle_keypress(key),
            Event::Mouse(_) => Ok(AppStateEvent::None),
            Event::Resize(_, _) => Ok(AppStateEvent::None),
            _ => Ok(AppStateEvent::None),
        }
    }

    fn handle_keypress(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) -> color_eyre::eyre::Result<super::appstate::AppStateEvent> {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                Ok(AppStateEvent::ExitState)
            }
            (_, KeyCode::Down | KeyCode::Char('j')) => Ok(AppStateEvent::None),
            (_, KeyCode::Up | KeyCode::Char('k')) => Ok(AppStateEvent::None),
            _ => Ok(AppStateEvent::None),
        }
    }

    fn pause(&mut self) {}

    fn unpause(&mut self) {}

    fn quit(&mut self) {}
}
