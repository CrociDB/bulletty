use clap::{Parser, Subcommand};
use tracing::info;

use crate::core::defs;
use crate::core::library::data::config::Config;
use crate::core::library::feedlibrary::FeedLibrary;
use std::path::Path;

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
    /// Show important directories
    Dirs,
}

pub fn run_main_cli(cli: Cli) -> color_eyre::Result<()> {
    info!("Initializing CLI");

    match &cli.command {
        Some(Commands::List) => command_list(&cli),
        Some(Commands::Add { url, category }) => command_add(&cli, url, category),
        Some(Commands::Update) => command_update(&cli),
        Some(Commands::Dirs) => command_dirs(&cli),
        None => Ok(()),
    }
}

fn command_list(_cli: &Cli) -> color_eyre::Result<()> {
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

fn command_add(_cli: &Cli, url: &str, category: &Option<String>) -> color_eyre::Result<()> {
    let mut library = FeedLibrary::new();
    let feed = library.add_feed_from_url(url, category)?;

    info!("Feed added: {feed:?}");
    println!("Feed added: {feed:?}");

    Ok(())
}

fn command_update(_cli: &Cli) -> color_eyre::Result<()> {
    let library = FeedLibrary::new();

    for category in library.feedcategories.iter() {
        for feed in category.feeds.iter() {
            info!("Updating {}", feed.title);
            println!("Updating {}", feed.title);
            library.data.update_feed_entries(category, feed, None)?;
        }
    }

    Ok(())
}

fn command_dirs(_cli: &Cli) -> color_eyre::Result<()> {
    let config = Config::new();
    let library_path = config.datapath;

    let logs_path = Path::new(&dirs::state_dir().unwrap()).join(defs::LOG_DIR);

    println!("bulletty directories");
    println!("\t-> Library: {}", library_path.to_string_lossy());
    println!("\t-> Logs: {}", logs_path.to_string_lossy());

    Ok(())
}
