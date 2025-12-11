use std::cell::RefCell;
use std::rc::Rc;

use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::layout::{Alignment, Constraint, Layout, Margin, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{
    Block, List, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap,
};

use crate::app::AppWorkStatus;

use crate::core::library::feedlibrary::FeedLibrary;
use crate::core::ui::appscreen::{AppScreen, AppScreenEvent};
use crate::core::ui::dialog::Dialog;
use crate::ui::states::themestate::ThemeState;

pub struct NewFeedDialog {
    library: Rc<RefCell<FeedLibrary>>,
    state: ThemeState,
}

impl NewFeedDialog {
    pub fn new(library: Rc<RefCell<FeedLibrary>>) -> Self {
        Self {
            library,
            state: ThemeState::default(),
        }
    }
}

impl Dialog for NewFeedDialog {
    fn get_size(&self) -> ratatui::prelude::Rect {
        Rect::new(70, 30, 0, 0)
    }

    fn as_screen(&self) -> &dyn AppScreen {
        self
    }

    fn as_screen_mut(&mut self) -> &mut dyn AppScreen {
        self
    }
}

impl AppScreen for NewFeedDialog {
    fn start(&mut self) {}

    fn quit(&mut self) {}

    fn pause(&mut self) {}

    fn unpause(&mut self) {}

    fn render(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        let theme = {
            let library = self.library.borrow();
            library.settings.get_theme().unwrap().clone()
        };

        let contentlayout = Layout::vertical([Constraint::Length(2), Constraint::Fill(1)])
            .split(area.inner(Margin::new(2, 1)));

        let title = Paragraph::new(self.get_title())
            .style(Style::new().fg(Color::from_u32(theme.base[0x8])))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(title, contentlayout[0]);
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
            (_, KeyCode::Esc | KeyCode::Char('q') | KeyCode::Enter)
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => {
                Ok(AppScreenEvent::CloseDialog)
            }
            (_, KeyCode::Down | KeyCode::Char('j')) => {
                self.state.select_next();
                let selected = self.state.get_selected();
                self.library.borrow_mut().settings.appearance.theme = selected.unwrap();
                self.library.borrow_mut().settings.appearance.save()?;
                Ok(AppScreenEvent::None)
            }
            (_, KeyCode::Up | KeyCode::Char('k')) => {
                self.state.select_previous();
                let selected = self.state.get_selected();
                self.library.borrow_mut().settings.appearance.theme = selected.unwrap();
                self.library.borrow_mut().settings.appearance.save()?;
                Ok(AppScreenEvent::None)
            }
            _ => Ok(AppScreenEvent::None),
        }
    }

    fn get_work_status(&self) -> AppWorkStatus {
        AppWorkStatus::None
    }

    fn get_title(&self) -> String {
        String::from("New Feed")
    }

    fn get_instructions(&self) -> String {
        String::from("Esc: close")
    }

    fn get_full_instructions(&self) -> String {
        self.get_instructions()
    }
}
