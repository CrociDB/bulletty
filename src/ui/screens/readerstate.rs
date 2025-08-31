use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{
    Block, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap,
};

use crate::app::AppWorkStatus;
use crate::core::{
    feed::feedentry::FeedEntry,
    ui::appscreen::{AppScreen, AppScreenEvent},
};

pub struct ReaderState {
    feedentry: FeedEntry,
    scroll: usize,
    scrollmax: usize,
}

impl ReaderState {
    pub fn new(entry: FeedEntry) -> ReaderState {
        ReaderState {
            feedentry: entry,
            scroll: 0,
            scrollmax: 1,
        }
    }

    pub fn scrollup(&mut self) {
        if self.scroll > 0 {
            self.scroll -= 1;
        }
    }

    pub fn scrolldown(&mut self) {
        self.scroll = std::cmp::min(self.scroll + 1, self.scrollmax);
    }
}

impl AppScreen for ReaderState {
    fn start(&mut self) {}

    fn render(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        let block = Block::default()
            .style(Style::default().bg(Color::from_u32(0x262626)))
            .padding(Padding::new(3, 3, 3, 3));

        frame.render_widget(block, area);

        let sizelayout = Layout::horizontal([
            Constraint::Min(1),
            Constraint::Percentage(60),
            Constraint::Max(3),
            Constraint::Fill(1),
        ])
        .margin(2)
        .split(area);

        let contentlayout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Fill(1),
        ])
        .split(sizelayout[1]);

        let title = Paragraph::new(self.feedentry.title.as_str())
            // .block(block)
            .style(Style::new().fg(Color::LightRed))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        let date = Paragraph::new(format!(
            "\u{f0520} {} | \u{f09e} {}",
            self.feedentry
                .date
                .with_timezone(&chrono::Local)
                .format("%Y-%m-%d"),
            self.feedentry.author
        ))
        .style(Style::new().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

        let textwidget = tui_markdown::from_str(&self.feedentry.text);
        self.scrollmax = textwidget.height();

        let text = Paragraph::new(textwidget)
            .scroll((self.scroll as u16, 0))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        frame.render_widget(title, contentlayout[0]);
        frame.render_widget(date, contentlayout[1]);
        frame.render_widget(text, contentlayout[2]);

        let mut scrollbarstate = ScrollbarState::new(self.scrollmax).position(self.scroll);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
        frame.render_stateful_widget(scrollbar, sizelayout[2], &mut scrollbarstate);
    }

    fn handle_events(&mut self) -> Result<AppScreenEvent> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.handle_keypress(key),
            Event::Mouse(_) => Ok(AppScreenEvent::None),
            Event::Resize(_, _) => Ok(AppScreenEvent::None),
            _ => Ok(AppScreenEvent::None),
        }
    }

    fn handle_keypress(
        &mut self,
        key: crossterm::event::KeyEvent,
    ) -> color_eyre::eyre::Result<AppScreenEvent> {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                Ok(AppScreenEvent::ExitState)
            }
            (_, KeyCode::Down | KeyCode::Char('j')) => {
                self.scrolldown();
                Ok(AppScreenEvent::None)
            }
            (_, KeyCode::Up | KeyCode::Char('k')) => {
                self.scrollup();
                Ok(AppScreenEvent::None)
            }
            _ => Ok(AppScreenEvent::None),
        }
    }

    fn pause(&mut self) {}

    fn unpause(&mut self) {}

    fn quit(&mut self) {}

    fn get_state_name(&self) -> String {
        String::from("Reader")
    }

    fn get_state_instructions(&self) -> String {
        String::from("j/k/↓/↑: scroll | Esc/q: leave")
    }

    fn get_state_work_status(&self) -> AppWorkStatus {
        AppWorkStatus::None
    }
}
