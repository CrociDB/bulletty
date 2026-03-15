use std::path::Path;
use tracing_appender::{non_blocking::WorkerGuard, rolling};

/// Initialize logging setup.
pub fn init(log_dir: &Path) -> Option<WorkerGuard> {
    let file_appender = rolling::daily(&log_dir, "app.log");
    let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_writer(non_blocking_appender)
        .init();

    Some(guard)
}
