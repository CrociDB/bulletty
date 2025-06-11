use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use toml;

use crate::defs;
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub datapath: PathBuf,
}

impl Config {
    pub fn new() -> Config {
        if let Some(config_dir) = dirs::config_dir() {
            let config_file = Path::new(&config_dir)
                .join(defs::CONFIG_PATH)
                .join(defs::CONFIG_FILE);

            if !config_file.exists() {
                let config_path = Path::new(&config_dir).join(defs::CONFIG_PATH);
                if !config_path.exists() {
                    if let Err(e) = fs::create_dir_all(config_path) {
                        eprintln!("Failed to create directory: {}", e);
                        std::process::exit(1);
                    }
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
                                eprintln!("Failed to write config: {}", e);
                                std::process::exit(1);
                            }

                            config
                        } else {
                            eprintln!("Error: data dir not found");
                            std::process::exit(1);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to create new config file: {}", e);
                        std::process::exit(1);
                    }
                }
            } else if let Ok(configstr) = std::fs::read_to_string(&config_file) {
                match toml::from_str(&configstr) {
                    Ok(config) => config,
                    Err(e) => {
                        eprintln!("Error: config file can't be parsed: {}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                eprintln!("Error: can't read config file");
                std::process::exit(1);
            }
        } else {
            eprintln!("Error: no config dir");
            std::process::exit(1);
        }
    }
}
