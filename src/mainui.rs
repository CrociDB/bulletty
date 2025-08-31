use tracing::info;

use crate::{app, ui::screens::mainscreen};

pub fn run_main_ui() -> color_eyre::Result<()> {
    info!("Initializing UI");

    let terminal = ratatui::init();

    let mut app = app::App::new();
    app.init(Box::new(mainscreen::MainScreen::new()));

    let result = app.run(terminal);
    ratatui::restore();
    result
}
