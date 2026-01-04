use crate::core::defs;
use std::path::{Path, PathBuf};
use tracing_appender::{non_blocking::WorkerGuard, rolling};

#[cfg(target_os = "windows")]
pub(crate) fn logging_dir() -> PathBuf {
    Path::new(&dirs::data_local_dir().unwrap()).join(defs::LOG_DIR)
}

#[cfg(not(target_os = "windows"))]
pub(crate) fn logging_dir() -> PathBuf {
    Path::new(&dirs::state_dir().unwrap()).join(defs::LOG_DIR)
}

pub fn init() -> Option<WorkerGuard> {
    if let Some(log_dir) = dirs::data_local_dir() {
        let log_dir = Path::new(&log_dir).join(defs::LOG_DIR);

        let file_appender = rolling::daily(&log_dir, "app.log");
        let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);

        tracing_subscriber::fmt()
            .with_writer(non_blocking_appender)
            .init();

        Some(guard)
    } else {
        None
    }
}
