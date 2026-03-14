pub mod app;
pub mod cli;
pub mod core;
pub mod logging;
pub mod mainui;
pub mod ui;

use clap::Parser;
use color_eyre::eyre::bail;

use crate::core::{
    config::{Config, ConfigStore},
    defs::PROGRAM_NAME,
};

pub fn run() -> color_eyre::Result<()> {
    let _guard = logging::init();
    color_eyre::install()?;

    let Some(config_dir) = dirs::config_dir().map(|base| base.join(PROGRAM_NAME)) else {
        bail!("Failed to find user configuration directory")
    };
    let Some(default_data_dir) = dirs::data_dir().map(|base| base.join(PROGRAM_NAME)) else {
        bail!("Failed to find user data directory")
    };

    let config_store = ConfigStore::new(&config_dir);
    let mut config = config_store.get_or_create(|| Config {
        datapath: default_data_dir,
    })?;

    let cli = cli::Cli::parse();

    if cli.command.is_none() {
        mainui::run_main_ui(&config)
    } else {
        cli::run_main_cli(cli, &mut config, &config_store)
    }
}

fn main() -> color_eyre::Result<()> {
    run()
}
