use tracing::info;

use crate::{app, ui::screens::mainstate};

pub fn run_main_ui() -> color_eyre::Result<()> {
    info!("Initializing UI");

    let terminal = ratatui::init();

    let mut app = app::App::new();
    app.init(Box::new(mainstate::MainState::new()));

    let result = app.run(terminal);
    ratatui::restore();
    result
}
