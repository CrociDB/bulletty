use std::path::{Path, PathBuf};

use color_eyre::eyre::{OptionExt, Result};
use etcetera::BaseStrategy;

use crate::core::defs::{CONFIG_PATH, DATA_DIR, LEGACY_LOG_DIR, LOG_BASE_DIR, LOG_SUBDIR};

pub struct Directories {
    log: PathBuf,
    default_data: PathBuf,
    config: PathBuf,
}

impl Directories {
    pub fn new() -> Result<Self> {
        let current_strategy = etcetera::choose_base_strategy()?;

        let legacy_log_dir = if cfg!(target_os = "linux") {
            dirs::state_dir().ok_or_eyre("Failed to get state directory for user")
        } else {
            dirs::data_local_dir().ok_or_eyre("Failed to get local data directory for user ")
        }
        .map(|base| base.join(LEGACY_LOG_DIR))?;

        let log_dir = if legacy_log_dir.try_exists()? {
            legacy_log_dir
        } else {
            current_strategy
                .state_dir() // Exists on Linux/macOS only
                .unwrap_or_else(|| current_strategy.cache_dir())
                .join(LOG_BASE_DIR)
                .join(LOG_SUBDIR)
        };

        let legacy_config_dir = dirs::config_dir()
            .ok_or_eyre("Failed to get configuration directory for user")
            .map(|base| base.join(CONFIG_PATH))?;
        let config_dir = if legacy_config_dir.try_exists()? {
            legacy_config_dir
        } else {
            current_strategy.config_dir().join(CONFIG_PATH)
        };

        Ok(Directories {
            log: log_dir,
            default_data: current_strategy.data_dir().join(DATA_DIR),
            config: config_dir,
        })
    }

    pub fn log(&self) -> &Path {
        &self.log
    }

    pub fn default_data(&self) -> &Path {
        &self.default_data
    }

    pub fn config(&self) -> &Path {
        &self.config
    }
}
