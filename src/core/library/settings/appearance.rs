use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tracing::error;

const APPEARANCE_PATH: &str = ".appearance.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct Appearance {
    #[serde(default = "default_tree_width")]
    pub main_screen_tree_width: u16,
    #[serde(default = "default_reader_width")]
    pub reader_width: u16,

    #[serde(skip)]
    path: PathBuf,
}

// Defaults
fn default_tree_width() -> u16 {
    30
}

fn default_reader_width() -> u16 {
    60
}

impl Appearance {
    pub fn new(datapath: &Path) -> color_eyre::Result<Self> {
        let path = datapath.join(APPEARANCE_PATH);

        if !path.exists() {
            let mut appearance: Self = toml::from_str("")?;
            appearance.path = path.clone();
            return Ok(appearance);
        }

        let data = fs::read_to_string(&path)?;
        let mut appearance: Appearance = match toml::from_str(&data) {
            Ok(a) => a,
            Err(e) => {
                error!("Error parsing {path:?}: {e:?}");
                toml::from_str("")?
            }
        };

        appearance.path = path.clone();
        Ok(appearance)
    }

    pub fn save(&mut self) -> color_eyre::Result<()> {
        let toml_string = toml::to_string_pretty(self)?;
        fs::write(&self.path, toml_string)?;
        Ok(())
    }
}
