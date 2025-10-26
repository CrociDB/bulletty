use std::{collections::HashMap, path::Path};

use crate::core::library::settings::{appearance::Appearance, theme::Theme, themedata};

pub struct UserSettings {
    pub appearance: Appearance,
    themes: HashMap<String, Theme>,
}

impl UserSettings {
    pub fn new(datapath: &Path) -> color_eyre::Result<Self> {
        Ok(Self {
            appearance: Appearance::new(datapath)?,
            themes: themedata::get_themes(),
        })
    }

    pub fn get_theme(&self) -> Option<&Theme> {
        if let Some(theme) = self.themes.get(&self.appearance.theme) {
            return Some(theme);
        }

        if let Some((_, value)) = self.themes.iter().next() {
            Some(value)
        } else {
            None
        }
    }

    pub fn get_theme_list(&self) -> Vec<String> {
        self.themes.keys().map(|t| t.to_string()).collect()
    }
}
