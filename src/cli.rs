use clap::{Parser, Subcommand};

use crate::library::feedlibrary::FeedLibrary;

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
    Add { url: String },
}

pub fn run_main_cli(cli: Cli) -> color_eyre::Result<()> {
    match &cli.command {
        Some(Commands::List) => command_list(&cli),
        Some(Commands::Add { url }) => command_add(&cli, url),
        None => Ok(()),
    }
}

fn command_list(cli: &Cli) -> Result<(), color_eyre::eyre::Error> {
    let library = FeedLibrary::new();

    println!("Feeds Registered\n\n");
    for category in library.feedcategories.iter() {
        println!("{}", category.title);
        for feed in category.feeds.iter().as_ref() {
            println!("\t-> {}", feed.title);
        }
        println!();
    }

    Ok(())
}

fn command_add(cli: &Cli, url: &str) -> Result<(), color_eyre::eyre::Error> {
    let mut library = FeedLibrary::new();
    let feed = library.add_feed(url)?;

    println!("Feed added: {:?}", feed);

    Ok(())
}
