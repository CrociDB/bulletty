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
    /// List all feeds and categories
    List,
    /// Add new feed
    Add {
        /// The ATOM/RSS feed URL
        url: String,
        #[arg()]
        /// The category to add under, if none is passed, it will be added to General
        category: Option<String>,
    },
    /// Update all feeds
    Update,
}

pub fn run_main_cli(cli: Cli) -> color_eyre::Result<()> {
    match &cli.command {
        Some(Commands::List) => command_list(&cli),
        Some(Commands::Add { url, category }) => command_add(&cli, url, category),
        Some(Commands::Update) => Ok(()),
        None => Ok(()),
    }
}

fn command_list(_cli: &Cli) -> Result<(), color_eyre::eyre::Error> {
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

fn command_add(_cli: &Cli, url: &str, category: &Option<String>) -> Result<(), color_eyre::eyre::Error> {
    let mut library = FeedLibrary::new();
    let feed = library.add_feed(url, category)?;

    println!("Feed added: {:?}", feed);

    Ok(())
}
