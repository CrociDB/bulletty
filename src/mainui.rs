use crate::{app, ui::readerstate};

pub fn run_main_ui()->color_eyre::Result<()> {
    let terminal = ratatui::init();

    let mut app = app::App::new();
    app.init(Box::new(readerstate::ReaderState::new()));

    let result = app.run(terminal);
    ratatui::restore();
    result
}
