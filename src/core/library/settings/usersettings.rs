use std::path::Path;

use crate::core::library::settings::appearance::Appearance;

pub struct UserSettings {
    pub appearance: Appearance,
}

impl UserSettings {
    pub fn new(datapath: &Path) -> color_eyre::Result<Self> {
        Ok(Self {
            appearance: Appearance::new(datapath)?,
        })
    }
}
