use clap::Parser;

mod core;
mod app;
mod cli;
mod logging;
mod mainui;
mod ui;

fn main() -> color_eyre::Result<()> {
    let _guard = logging::init();
    color_eyre::install()?;

    let cli = cli::Cli::parse();

    if cli.command.is_none() {
        mainui::run_main_ui()
    } else {
        cli::run_main_cli(cli)
    }
}
