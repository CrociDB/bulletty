use crate::core::defs;
use std::path::{Path, PathBuf};
use tracing_appender::{non_blocking::WorkerGuard, rolling};

/// Get the logging directory for the app if it exists.
#[cfg(target_os = "linux")]
pub(crate) fn logging_dir() -> Option<PathBuf> {
    dirs::state_dir().map(|log_base_dir| Path::new(&log_base_dir).join(defs::LOG_DIR))
}

#[cfg(not(target_os = "linux"))]
pub(crate) fn logging_dir() -> Option<PathBuf> {
    dirs::data_local_dir().map(|log_base_dir| Path::new(&log_base_dir).join(defs::LOG_DIR))
}

/// Initialize logging setup.
pub fn init() -> Option<WorkerGuard> {
    if let Some(log_dir) = logging_dir() {
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
