use std::{
    fs,
    path::{Path, PathBuf},
};
use toml::{Table, Value};
use tracing::error;

const APPEARANCE_PATH: &str = ".appearance.toml";
const DEFAULT_CONFIG: &str = r#"main_screen_tree_width = 30
reader_width = 50
"#;

pub struct Appearance {
    pub data: Table,
    path: PathBuf,
}

impl Appearance {
    pub fn new(datapath: &Path) -> Self {
        let path = datapath.join(APPEARANCE_PATH);

        let d = match load_appearance(&path) {
            Ok(d) => d,
            Err(e) => {
                error!("There's an error with the appearance file: {e:?}. Creating a default one.");
                match generate_appearance() {
                    Ok(d) => d,
                    Err(e) => {
                        std::panic!("Couldn't generate a default appearance file: \n\n{e}");
                    }
                }
            }
        };

        Self { data: d, path }
    }

    pub fn save(&mut self) -> color_eyre::Result<()> {
        save_appearance(&self.path, &self.data)
    }
}

fn generate_appearance() -> color_eyre::Result<Table> {
    let table = DEFAULT_CONFIG.parse::<Table>()?;
    Ok(table)
}

fn load_appearance(path: &Path) -> color_eyre::Result<Table> {
    if let Ok(r) = path.try_exists()
        && r
    {
        let toml_str = fs::read_to_string(path)?;
        let table = toml_str.parse::<Table>()?;
        Ok(table)
    } else {
        Err(color_eyre::eyre::eyre!(
            "Appearance config file doesn't exist: {path:?}"
        ))
    }
}

fn save_appearance(path: &PathBuf, table: &toml::Table) -> color_eyre::Result<()> {
    let value = Value::Table(table.clone());
    let toml_string = toml::to_string_pretty(&value)?;
    fs::write(path, toml_string)?;
    Ok(())
}
