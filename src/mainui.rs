use tracing::info;

use crate::{app, core::config::Config};

pub fn run_main_ui(config: &Config) -> color_eyre::Result<()> {
    info!("Initializing UI");

    if let Some(hooks) = &config.hooks {
        hooks.run_before_tui();
    }

    let terminal = ratatui::init();
    let mut app = app::App::new(config);
    app.initmain();
    let result = app.run(terminal);
    ratatui::restore();

    if let Some(hooks) = &config.hooks {
        hooks.run_after_tui();
    }

    result
}
