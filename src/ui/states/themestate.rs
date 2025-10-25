use ratatui::widgets::{ListItem, ListState};

use crate::core::library::feedlibrary::FeedLibrary;

#[derive(Default)]
pub struct ThemeState {
    pub themes: Vec<String>,
    pub state: ListState,
}

impl ThemeState {
    pub fn update(&mut self, library: &FeedLibrary) {
        self.themes = library.settings.get_theme_list();
        self.themes.sort();

        let selected = &library.settings.appearance.theme;
        if let Some(id) = self
            .themes
            .iter()
            .position(|theme| theme.as_str() == selected)
        {
            self.state.select(Some(id));
        } else {
            self.state.select(Some(0))
        }
    }

    pub fn get_items(&self) -> Vec<ListItem<'_>> {
        self.themes
            .iter()
            .map(|t| ListItem::new(t.to_string()))
            .collect()
    }

    pub fn get_selected(&self) -> Option<String> {
        if !self.themes.is_empty() {
            Some(self.themes[self.state.selected().unwrap_or(0)].to_string())
        } else {
            None
        }
    }

    pub fn select_next(&mut self) {
        if self.themes.is_empty() {
            return;
        }

        let selected = self.state.selected().unwrap_or(0);
        if selected < self.themes.len().saturating_sub(1) {
            self.state.select_next();
        }
    }

    pub fn select_previous(&mut self) {
        if self.themes.is_empty() {
            return;
        }

        let selected = self.state.selected().unwrap_or(0);
        if selected >= self.themes.len() {
            self.state.select(Some(self.themes.len().saturating_sub(1)));
        }

        let selected = self.state.selected().unwrap_or(0);

        if selected > 0 {
            self.state.select_previous();
        }
    }

    pub fn select_first(&mut self) {
        if self.themes.is_empty() {
            return;
        }

        self.state.select_first();
    }

    pub fn select_last(&mut self) {
        if self.themes.is_empty() {
            return;
        }

        self.state.select(Some(self.themes.len().saturating_sub(1)));
    }
}
