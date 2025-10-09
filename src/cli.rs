use clap::{Parser, Subcommand};
use tracing::{error, info};

use crate::core::defs;
use crate::core::library::data::config::Config;
use crate::core::library::data::opml;
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
    /// Import a list of feed sources through OPML
    Import {
        /// The filepath of the OPML file
        opml_file: String,
    },
    /// Export all your sources to an OPML file
    Export {
        /// The filepath of the OPML file
        opml_file: String,
    },
}

pub fn run_main_cli(cli: Cli) -> color_eyre::Result<()> {
    info!("Initializing CLI");

    match &cli.command {
        Some(Commands::List) => command_list(&cli),
        Some(Commands::Add { url, category }) => command_add(&cli, url, category),
        Some(Commands::Update) => command_update(&cli),
        Some(Commands::Dirs) => command_dirs(&cli),
        Some(Commands::Import { opml_file }) => command_import(&cli, opml_file),
        Some(Commands::Export { opml_file }) => command_export(&cli, opml_file),
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
    match library.add_feed_from_url(url, category) {
        Ok(feed) => {
            info!("Feed added: {}", feed.title);
            println!("Feed added: {}", feed.title);
        }
        Err(err) => {
            error!("{}", err);
            println!("{}", err);
        }
    }

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
    println!("\t-> Logs:    {}", logs_path.to_string_lossy());

    Ok(())
}

fn command_import(_cli: &Cli, opml_file: &str) -> color_eyre::Result<()> {
    println!("Importing feeds");
    let mut library = FeedLibrary::new();
    let opml_feeds = opml::get_opml_feeds(opml_file)?;

    for feed in opml_feeds {
        match library.add_feed_from_url(&feed.url, &feed.category) {
            Ok(feed) => {
                info!("Feed added: {feed:?}");
                println!("Feed added: {feed:?}");
            }
            Err(err) => {
                error!("{}", err);
                println!("{}", err);
            }
        }
    }

    Ok(())
}

fn command_export(_cli: &Cli, opml_file: &str) -> color_eyre::Result<()> {
    let library = FeedLibrary::new();

    opml::save_opml(&library.feedcategories, opml_file)?;

    Ok(())
}
