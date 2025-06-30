use crate::{app, ui::mainstate};

pub fn run_main_ui()->color_eyre::Result<()> {
    let terminal = ratatui::init();

    let mut app = app::App::new();
    app.init(Box::new(mainstate::MainState::new()));

    let result = app.run(terminal);
    ratatui::restore();
    result
}
