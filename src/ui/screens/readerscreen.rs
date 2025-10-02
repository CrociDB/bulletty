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
    // New fields for navigation
    entries: Vec<FeedEntry>,
    current_index: usize,
}

impl ReaderScreen {
    pub fn new(entry: FeedEntry) -> ReaderScreen {
        ReaderScreen {
            feedentry: entry,
            scroll: 0,
            scrollmax: 1,
            entries: vec![],
            current_index: 0,
        }
    }

    // New constructor that accepts a list of entries and current index
    pub fn new_with_entries(entries: Vec<FeedEntry>, current_index: usize) -> ReaderScreen {
        let entry = if current_index < entries.len() {
            entries[current_index].clone()
        } else {
            entries.first().cloned().unwrap_or_default()
        };

        ReaderScreen {
            feedentry: entry,
            scroll: 0,
            scrollmax: 1,
            entries,
            current_index,
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

    // Navigate to next entry
    fn go_to_next_entry(&mut self) {
        if !self.entries.is_empty() && self.current_index < self.entries.len() - 1 {
            self.current_index += 1;
            self.feedentry = self.entries[self.current_index].clone();
            self.scroll = 0; // Reset scroll position
        }
    }

    // Navigate to previous entry
    fn go_to_previous_entry(&mut self) {
        if !self.entries.is_empty() && self.current_index > 0 {
            self.current_index -= 1;
            self.feedentry = self.entries[self.current_index].clone();
            self.scroll = 0; // Reset scroll position
        }
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

        // Title with navigation indicator
        let title_text = if !self.entries.is_empty() {
            format!("{} ({}/{})", 
                self.feedentry.title, 
                self.current_index + 1, 
                self.entries.len())
        } else {
            self.feedentry.title.clone()
        };
        
        let title = Paragraph::new(title_text.as_str())
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
        let url = Paragraph::new(format!("\u{f054a} {}", self.feedentry.url))
            .style(Style::new().fg(Color::from_u32(0x777777)))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(url, contentlayout[2]);

        // Content
        let content = Paragraph::new(self.feedentry.content.as_str())
            .style(Style::new().fg(Color::from_u32(0xdddddd)))
            .wrap(Wrap { trim: false })
            .scroll((self.scroll as u16, 0));

        frame.render_widget(content, contentlayout[3]);

        let paragraph_height = contentlayout[3].height as usize;
        let text_height = self
            .feedentry
            .content
            .lines()
            .map(|line| {
                let line_width = line.width();
                let area_width = contentlayout[3].width as usize;
                if area_width == 0 {
                    1
                } else {
                    std::cmp::max(1, (line_width + area_width - 1) / area_width)
                }
            })
            .sum::<usize>();

        self.scrollmax = if text_height > paragraph_height {
            text_height - paragraph_height
        } else {
            0
        };

        if text_height > paragraph_height {
            let mut scrollbar_state = ScrollbarState::default()
                .content_length(text_height)
                .viewport_content_length(paragraph_height)
                .position(self.scroll);

            let scrollbar = Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None);

            frame.render_stateful_widget(scrollbar, sizelayout[2], &mut scrollbar_state);
        }
    }

    fn handle_event(&mut self, event: Event) -> Result<AppScreenEvent> {
        if let Event::Key(key) = event {
            if key.kind != KeyEventKind::Press {
                return Ok(AppScreenEvent::None);
            }

            match (key.modifiers, key.code) {
                (_, KeyCode::Char('q'))
                | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C'))
                | (_, KeyCode::Esc) => Ok(AppScreenEvent::ExitState),
                (_, KeyCode::Down | KeyCode::Char('j')) => {
                    self.scrolldown();
                    Ok(AppScreenEvent::None)
                }
                (_, KeyCode::Up | KeyCode::Char('k')) => {
                    self.scrollup();
                    Ok(AppScreenEvent::None)
                }
                (_, KeyCode::Char('o')) => self.open_external_url(&self.feedentry.url),
                (_, KeyCode::Char('?')) => Ok(AppScreenEvent::OpenDialog(Box::new(
                    HelpDialog::new(self.get_full_instructions()),
                ))),
                // New navigation keys
                (_, KeyCode::Char('n')) => {
                    self.go_to_next_entry();
                    Ok(AppScreenEvent::None)
                }
                (_, KeyCode::Char('p')) => {
                    self.go_to_previous_entry();
                    Ok(AppScreenEvent::None)
                }
                _ => Ok(AppScreenEvent::None),
            }
        } else {
            Ok(AppScreenEvent::None)
        }
    }

    fn get_title(&self) -> String {
        String::from("Reader")
    }

    fn get_instructions(&self) -> String {
        if !self.entries.is_empty() {
            String::from("?: Help | j/k/↓/↑: scroll | n/p: next/prev entry | o: open | Esc: back")
        } else {
            String::from("?: Help | j/k/↓/↑: scroll | o: open | Esc: back")
        }
    }

    fn get_work_status(&self) -> AppWorkStatus {
        AppWorkStatus::Working
    }

    fn get_full_instructions(&self) -> String {
        String::from(
            r#"Reader Screen Instructions:

j/k or ↓/↑   - Scroll content up/down
n            - Go to next entry (when available)
p            - Go to previous entry (when available)
o            - Open URL in external browser
?            - Show this help
Esc/q        - Exit reader mode

Navigation:
When viewing entries from a feed category, you can navigate
between entries using 'n' (next) and 'p' (previous) without
leaving reader mode. The title shows current position (x/y).
"#,
        )
    }
}

// Add Default implementation for FeedEntry if it doesn't exist
impl Default for FeedEntry {
    fn default() -> Self {
        FeedEntry {
            title: String::from("No Title"),
            content: String::from("No Content"),
            url: String::from(""),
            author: String::from("Unknown"),
            date: chrono::Utc::now(),
            seen: false,
        }
    }
}
