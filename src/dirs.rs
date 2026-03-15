use std::path::{Path, PathBuf};

use crate::core::defs::PROGRAM_NAME;

pub struct Directories {
    log: PathBuf,
    default_data: PathBuf,
    config: PathBuf,
}

impl Directories {
    pub fn new() -> Option<Self> {
        Some(Directories {
            log: if cfg!(target_os = "linux") {
                dirs::state_dir()
            } else {
                dirs::data_local_dir()
            }?
            .join("bulletty_logs"),
            default_data: dirs::data_dir()?.join(PROGRAM_NAME),
            config: dirs::config_dir()?.join(PROGRAM_NAME),
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
