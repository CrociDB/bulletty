pub mod app;
pub mod cli;
pub mod core;
pub mod logging;
pub mod mainui;
pub mod ui;

use clap::Parser;

use crate::core::Config;

pub fn run() -> color_eyre::Result<()> {
    let _guard = logging::init();
    color_eyre::install()?;

    let mut config = Config::new();

    let cli = cli::Cli::parse();

    if cli.command.is_none() {
        mainui::run_main_ui(&config)
    } else {
        cli::run_main_cli(cli, &mut config)
    }
}

fn main() -> color_eyre::Result<()> {
    run()
}
