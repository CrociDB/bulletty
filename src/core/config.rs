use color_eyre::eyre::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub datapath: PathBuf,
}

pub struct ConfigStore {
    file_path: PathBuf,
}

impl ConfigStore {
    pub fn new(dir: &Path) -> Self {
        ConfigStore {
            file_path: dir.join("config.toml"),
        }
    }

    pub fn get_or_create(&self, default_config: impl FnOnce() -> Config) -> Result<Config> {
        if let Some(parent) = self.file_path.parent() {
            std::fs::create_dir_all(parent).wrap_err_with(|| {
                format!(
                    "Failed to create base configuration directory {}",
                    parent.to_string_lossy()
                )
            })?;
        }

        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.file_path)
        {
            Ok(mut file) => {
                Self::write(&default_config(), &mut file)?;
                Ok(())
            }
            Err(err) => match err.kind() {
                ErrorKind::AlreadyExists => Ok(()),
                _ => Err(err),
            },
        }
        .wrap_err_with(|| {
            format!(
                "Failed to create default configuration file at {}",
                self.file_path.to_string_lossy()
            )
        })?;

        self.read().wrap_err_with(|| {
            format!(
                "Failed to read configuration from {}",
                self.file_path.to_string_lossy()
            )
        })
    }

    pub fn save(&self, config: &Config) -> Result<()> {
        File::create(&self.file_path)
            .and_then(|mut file| Self::write(config, &mut file))
            .wrap_err_with(|| {
                format!(
                    "Failed to save configuration to {}",
                    self.file_path.to_string_lossy()
                )
            })
    }

    fn read(&self) -> Result<Config> {
        Ok(toml::from_str(&std::fs::read_to_string(&self.file_path)?)?)
    }

    fn write(config: &Config, file: &mut File) -> std::io::Result<()> {
        file.write_all(
            &toml::to_string(config)
                .expect("configuration should be serializable")
                .into_bytes(),
        )
    }
}
