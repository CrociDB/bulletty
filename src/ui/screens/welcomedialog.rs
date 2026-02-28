use std::{cell::RefCell, rc::Rc};

use color_eyre::eyre::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::{Alignment, Constraint, Layout, Margin, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Paragraph, Wrap};

use crate::app::AppWorkStatus;
use crate::core::library::feedlibrary::FeedLibrary;
use crate::core::ui::appscreen::{AppScreen, AppScreenEvent};
use crate::core::ui::dialog::Dialog;

pub struct WelcomeDialog {
    library: Rc<RefCell<FeedLibrary>>,
}

impl WelcomeDialog {
    pub fn new(library: Rc<RefCell<FeedLibrary>>) -> Self {
        Self { library }
    }
}

impl Dialog for WelcomeDialog {
    fn get_size(&self) -> Rect {
        Rect::new(70, 18, 0, 0)
    }

    fn as_screen(&self) -> &dyn AppScreen {
        self
    }

    fn as_screen_mut(&mut self) -> &mut dyn AppScreen {
        self
    }
}

impl AppScreen for WelcomeDialog {
    fn start(&mut self) {}

    fn quit(&mut self) {}

    fn pause(&mut self) {}

    fn unpause(&mut self) {}

    fn render(&mut self, frame: &mut ratatui::Frame, area: Rect) {
        let theme = {
            let library = self.library.borrow();
            library.settings.get_theme().unwrap().clone()
        };

        let layout = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Length(2),
        ])
        .split(area.inner(Margin::new(3, 1)));

        let title = Paragraph::new("Welcome to bulletty!")
            .style(Style::new().fg(Color::from_u32(theme.base[0xa])))
            .alignment(Alignment::Center);

        let subtitle = Paragraph::new("Your library is empty. Add your first feed to get started.")
            .style(Style::new().fg(Color::from_u32(theme.base[0x5])))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        let instructions_text = concat!(
            "To add a feed, run the following command from your terminal:\n\n",
            "  bulletty add <feed_url> [category]\n\n",
            "Example:\n\n",
            "  bulletty add https://crocidb.com/index.xml Programming\n\n",
            "The category is optional. If omitted, the feed will be placed\n",
            "in the default category: General."
        );

        let instructions = Paragraph::new(instructions_text)
            .style(Style::new().fg(Color::from_u32(theme.base[0x6])))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        let hint = Paragraph::new("Press Esc or q to dismiss")
            .style(Style::new().fg(Color::from_u32(theme.base[0x3])))
            .alignment(Alignment::Center);

        frame.render_widget(title, layout[0]);
        frame.render_widget(subtitle, layout[1]);
        frame.render_widget(instructions, layout[2]);
        frame.render_widget(hint, layout[4]);
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
            (_, KeyCode::Esc | KeyCode::Char('q') | KeyCode::Enter)
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
        String::from("Welcome")
    }

    fn get_instructions(&self) -> String {
        String::from("Esc/q/Enter: dismiss")
    }

    fn get_full_instructions(&self) -> String {
        self.get_instructions()
    }
}
