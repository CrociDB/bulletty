mod ui;
mod app;

use crate::ui::readerstate;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let mut app = app::App::new();
    app.init(Box::new(readerstate::ReaderState::new()));

    let result = app.run(terminal);
    ratatui::restore();
    result
}

