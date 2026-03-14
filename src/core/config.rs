use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use toml;
use tracing::error;

use crate::core::defs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub datapath: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn save(&self) {
        if let Some(config_dir) = dirs::config_dir() {
            let config_file = Path::new(&config_dir)
                .join(defs::CONFIG_PATH)
                .join(defs::CONFIG_FILE);

            match toml::to_string(self) {
                Ok(toml_string) => {
                    if let Err(e) = fs::write(&config_file, toml_string) {
                        error!("Failed to write config file: {}", e);
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    error!("Failed to serialize config: {}", e);
                    std::process::exit(1);
                }
            }
        } else {
            error!("No config dir");
            std::process::exit(1);
        }
    }

    pub fn new() -> Self {
        if let Some(config_dir) = dirs::config_dir() {
            let config_file = Path::new(&config_dir)
                .join(defs::CONFIG_PATH)
                .join(defs::CONFIG_FILE);

            if !config_file.exists() {
                let config_path = Path::new(&config_dir).join(defs::CONFIG_PATH);
                if !config_path.exists()
                    && let Err(e) = fs::create_dir_all(config_path)
                {
                    error!("Failed to create directory: {}", e);
                    std::process::exit(1);
                }

                match OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&config_file)
                {
                    Ok(mut file) => {
                        if let Some(data_dir) = dirs::data_dir() {
                            let config = Config {
                                datapath: data_dir.join(defs::DATA_DIR),
                            };

                            if let Err(e) =
                                file.write_all(&toml::to_string(&config).unwrap().into_bytes())
                            {
                                error!("Failed to write config: {}", e);
                                std::process::exit(1);
                            }

                            config
                        } else {
                            error!("Error: data dir not found");
                            std::process::exit(1);
                        }
                    }
                    Err(e) => {
                        error!("Failed to create new config file: {}", e);
                        std::process::exit(1);
                    }
                }
            } else if let Ok(configstr) = std::fs::read_to_string(&config_file) {
                match toml::from_str(&configstr) {
                    Ok(config) => config,
                    Err(e) => {
                        error!("Config file can't be parsed: {}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                error!("Can't read config file");
                std::process::exit(1);
            }
        } else {
            error!("No config dir");
            std::process::exit(1);
        }
    }
}
