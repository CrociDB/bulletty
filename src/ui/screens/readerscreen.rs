use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{
    Block, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap,
};
use tracing::error;
use unicode_width::UnicodeWidthStr;

use crate::app::AppWorkStatus;
use crate::core::{
    feed::feedentry::FeedEntry,
    ui::appscreen::{AppScreen, AppScreenEvent},
};
use crate::ui::screens::urldialog::UrlDialog;

use super::helpdialog::HelpDialog;

pub struct ReaderScreen {
    feedentry: FeedEntry,
    scroll: usize,
    scrollmax: usize,
}

impl ReaderScreen {
    pub fn new(entry: FeedEntry) -> ReaderScreen {
        ReaderScreen {
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

    fn open_external_url(&self, url: &str) -> Result<AppScreenEvent> {
        match open::that(url) {
            Ok(_) => Ok(AppScreenEvent::None),
            Err(_) => {
                error!("Couldn't invoke system browser");
                Ok(AppScreenEvent::OpenDialog(Box::new(UrlDialog::new(
                    url.to_string(),
                ))))
            }
        }
    }
}

impl AppScreen for ReaderScreen {
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
            Constraint::Length(1), // Title
            Constraint::Length(1), // Date
            Constraint::Length(2), // URL
            Constraint::Fill(1),   // Content
        ])
        .split(sizelayout[1]);

        // Title
        let title = Paragraph::new(self.feedentry.title.as_str())
            .style(Style::new().fg(Color::LightRed))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(title, contentlayout[0]);

        // Date
        let date = Paragraph::new(format!(
            "\u{f0520} {} | \u{f09e} {}",
            self.feedentry
                .date
                .with_timezone(&chrono::Local)
                .format("%Y-%m-%d"),
            self.feedentry.author
        ))
        .style(Style::new().fg(Color::from_u32(0x777777)))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

        frame.render_widget(date, contentlayout[1]);

        // URL
        let date = Paragraph::new(self.feedentry.url.to_string())
            .style(Style::new().fg(Color::Blue))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(date, contentlayout[2]);

        // Content
        let text = tui_markdown::from_str(&self.feedentry.text);
        let textheight = text.height() as usize;

        // This is a workaround to get more or less the amount of wrapped lines, to be used on the
        // scrollbar
        let mut wrapped_lines = 0;
        for line in text.lines.iter() {
            let content: String = line
                .spans
                .iter()
                .map(|span| span.content.to_string())
                .collect();
            let line_width = UnicodeWidthStr::width(content.as_str());
            let wrapped = line_width.div_ceil(contentlayout[3].width as usize);
            wrapped_lines += wrapped - wrapped.min(1);
        }

        let scrollheight = textheight + (wrapped_lines as f32 * 1.06) as usize + 4;
        self.scrollmax = scrollheight - (contentlayout[3].height as usize).min(scrollheight);

        // Content Paragraph component
        let paragraph = Paragraph::new(text)
            .scroll((self.scroll as u16, 0))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, contentlayout[3]);

        // Scrollbar
        let mut scrollbarstate = ScrollbarState::new(self.scrollmax).position(self.scroll);
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .style(Style::new().fg(Color::from_u32(0x444444)));
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
            (_, KeyCode::Home | KeyCode::Char('g')) => {
                self.scroll = 0;
                Ok(AppScreenEvent::None)
            }
            (_, KeyCode::End | KeyCode::Char('G')) => {
                self.scroll = self.scrollmax;
                Ok(AppScreenEvent::None)
            }
            (_, KeyCode::Char('o')) => self.open_external_url(&self.feedentry.url),
            (_, KeyCode::Char('?')) => Ok(AppScreenEvent::OpenDialog(Box::new(HelpDialog::new(
                self.get_full_instructions(),
            )))),
            _ => Ok(AppScreenEvent::None),
        }
    }

    fn pause(&mut self) {}

    fn unpause(&mut self) {}

    fn quit(&mut self) {}

    fn get_title(&self) -> String {
        String::from("Reader")
    }

    fn get_instructions(&self) -> String {
        String::from("j/k/↓/↑: scroll | o: open externally | Esc/q: leave")
    }

    fn get_work_status(&self) -> AppWorkStatus {
        AppWorkStatus::None
    }

    fn get_full_instructions(&self) -> String {
        String::from(
            "j/k/↓/↑: scroll\ng/G: go to beginning or end of file\n\no: open externally\n\nEsc/q: leave",
        )
    }
}
