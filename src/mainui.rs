use tracing::info;

use crate::app;

pub fn run_main_ui() -> color_eyre::Result<()> {
    info!("Initializing UI");

    let terminal = ratatui::init();

    let mut app = app::App::new();
    app.initmain();

    let result = app.run(terminal);
    ratatui::restore();
    result
}
