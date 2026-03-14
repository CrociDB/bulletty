use tracing::info;

use crate::{app, core::config::Config};

pub fn run_main_ui(config: &Config) -> color_eyre::Result<()> {
    info!("Initializing UI");

    let terminal = ratatui::init();

    let mut app = app::App::new(config);
    app.initmain();

    let result = app.run(terminal);
    ratatui::restore();
    result
}
