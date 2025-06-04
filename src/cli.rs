use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bulletty")]
#[command(version, about = "Your TUI feed reader", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    List,
    Add,
}

pub fn run_main_cli(cli: Cli) -> color_eyre::Result<()> {
    match &cli.command {
        Some(Commands::List) => command_list(&cli),
        Some(Commands::Add) => command_add(&cli),
        None => Ok(()),
    }
}

fn command_list(cli: &Cli) -> Result<(), color_eyre::eyre::Error> {
    todo!()
}

fn command_add(cli: &Cli) -> Result<(), color_eyre::eyre::Error> {
    Ok(())
}
