use crate::core::defs;
use std::path::Path;
use tracing_appender::{non_blocking::WorkerGuard, rolling};

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
