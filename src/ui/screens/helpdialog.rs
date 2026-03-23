use std::{cell::RefCell, rc::Rc};

use color_eyre::eyre::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::{Alignment, Constraint, Layout, Margin, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap};

use crate::app::AppWorkStatus;
use crate::core::library::feedlibrary::FeedLibrary;
use crate::core::ui::appscreen::{AppScreen, AppScreenEvent};
use crate::core::ui::dialog::Dialog;
use crate::core::ui::instructiondetails::ScreenInstructions;

pub struct HelpDialog {
    library: Rc<RefCell<FeedLibrary>>,
    instructions: ScreenInstructions,
    active_tab: usize,
    scroll: u16,
    scrollmax: u16,
}

impl HelpDialog {
    pub fn new(library: Rc<RefCell<FeedLibrary>>, instructions: ScreenInstructions) -> HelpDialog {
        HelpDialog {
            library,
            instructions,
            active_tab: 0,
            scroll: 0,
            scrollmax: 0,
        }
    }
}

impl Dialog for HelpDialog {
    fn get_size(&self) -> ratatui::prelude::Rect {
        Rect::new(70, 28, 0, 0)
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
        let theme = {
            let library = self.library.borrow();
            library.settings.get_theme().unwrap().clone()
        };

        let contentlayout = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .split(area.inner(Margin::new(2, 1)));

        let tab_labels = ["Instructions", "About"];
        let tab_chunks =
            Layout::horizontal(tab_labels.map(|_| Constraint::Fill(1))).split(contentlayout[0]);

        for (i, label) in tab_labels.iter().enumerate() {
            let (fg, bg) = if i == self.active_tab {
                (
                    Color::from_u32(theme.base[0x0]),
                    Color::from_u32(theme.base[0x8]),
                )
            } else {
                (
                    Color::from_u32(theme.base[0x6]),
                    Color::from_u32(theme.base[0x2]),
                )
            };
            let tab = Paragraph::new(*label)
                .style(Style::new().fg(fg).bg(bg))
                .alignment(Alignment::Center)
                .block(ratatui::widgets::Block::new().padding(ratatui::widgets::Padding::top(1)));
            frame.render_widget(tab, tab_chunks[i]);
        }

        let content_area = contentlayout[2];

        if self.active_tab == 0 {
            // Instructions tab with scrollbar
            let chunks = Layout::horizontal([Constraint::Fill(1), Constraint::Length(1)])
                .split(content_area);

            let mut lines: Vec<Line> = Vec::new();
            for category in &self.instructions.categories {
                lines.push(Line::from(Span::styled(
                    category.name.clone(),
                    Style::new()
                        .fg(Color::from_u32(theme.base[0x8]))
                        .add_modifier(Modifier::BOLD),
                )));
                lines.push(Line::from(""));
                for detail in &category.details {
                    lines.push(Line::from(vec![
                        Span::styled(
                            format!("  {:19}", detail.keys),
                            Style::new().fg(Color::from_u32(theme.base[0x6])),
                        ),
                        Span::styled(
                            detail.description.clone(),
                            Style::new().fg(Color::from_u32(theme.base[0x4])),
                        ),
                    ]));
                }
                lines.push(Line::from(""));
            }
            let content_text = Text::from(lines);
            self.scrollmax = (content_text.height() as u16).saturating_sub(chunks[0].height);
            let content = Paragraph::new(content_text)
                .alignment(Alignment::Left)
                .scroll((self.scroll, 0));

            frame.render_widget(content, chunks[0]);

            let mut scrollbar_state =
                ScrollbarState::new(self.scrollmax as usize).position(self.scroll as usize);
            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .style(Style::new().fg(Color::from_u32(theme.base[3])));
            frame.render_stateful_widget(scrollbar, chunks[1], &mut scrollbar_state);
        } else {
            // About tab
            let about_text = format!(
                "bulletty v{}\n\n{}",
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_AUTHORS")
            );
            let about = Paragraph::new(about_text)
                .style(Style::new().fg(Color::from_u32(theme.base[0x6])))
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });

            frame.render_widget(about, content_area);
        }
    }

    fn handle_event(&mut self, event: Event) -> Result<AppScreenEvent> {
        match event {
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
            (_, KeyCode::Tab) => {
                self.active_tab = (self.active_tab + 1) % 2;
                self.scroll = 0;
                Ok(AppScreenEvent::None)
            }
            (_, KeyCode::BackTab) => {
                self.active_tab = (self.active_tab - 1) % 2;
                self.scroll = 0;
                Ok(AppScreenEvent::None)
            }
            (_, KeyCode::Down | KeyCode::Char('j')) => {
                if self.active_tab == 0 {
                    self.scroll = self.scroll.saturating_add(1).min(self.scrollmax);
                }
                Ok(AppScreenEvent::None)
            }
            (_, KeyCode::Up | KeyCode::Char('k')) => {
                if self.active_tab == 0 {
                    self.scroll = self.scroll.saturating_sub(1);
                }
                Ok(AppScreenEvent::None)
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
        String::from("Tab: switch tabs | j/k ↑↓: scroll | Esc/q: close")
    }

    fn get_full_instructions(&self) -> ScreenInstructions {
        ScreenInstructions::empty()
    }
}
