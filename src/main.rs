use clap::Parser;

mod app;
mod cli;
mod defs;
mod feed;
mod library;
mod mainui;
mod ui;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = cli::Cli::parse();

    if cli.command.is_none() {
        mainui::run_main_ui()
    } else {
        cli::run_main_cli(cli)
    }
}
