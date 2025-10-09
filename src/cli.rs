use std::io::{self, Write};

use clap::{Error, Parser, Subcommand};
use tracing::{error, info};

use crate::core::defs;
use crate::core::library::data::config::Config;
use crate::core::library::data::opml;
use crate::core::library::feeditem::FeedItem;
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
    /// Delete a feed
    Delete {
        /// The feed identifier (can be url, title or slug)
        ident: String,
    },
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
        Some(Commands::Delete { ident }) => command_delete(&cli, ident),
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
            println!("\t-> {}: {}", feed.title, feed.slug);
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

fn confirm_delete(title: &str) -> Result<bool, Error> {
    print!(
        "Are you sure you want to delete '{}'? That can't be reverted. [y/N] ",
        title
    );
    io::stdout().flush()?;

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;

    let normalized_input = choice.trim().to_lowercase();
    Ok(matches!(normalized_input.as_str(), "y" | "yes"))
}

fn command_delete(_cli: &Cli, ident: &str) -> color_eyre::Result<()> {
    let library = FeedLibrary::new();

    let matches: Vec<&FeedItem> = library.get_matching_feeds(ident);
    let matches_len = matches.len();

    match matches_len {
        0 => {
            info!("No matching feeds exist");
            println!("No matching feeds exist");
        }
        1 => {
            let matched = matches[0];
            if confirm_delete(&matched.title)? {
                library.delete_feed(&matched.slug, &matched.category)?;
                info!("Feed deleted: {}", &matched.title);
                println!("Feed deleted: {}", &matched.title);
            } else {
                info!("Feed was not deleted: {}", &matched.title);
                println!("Feed was not deleted: {}", &matched.title);
            }
        }
        _ => {
            println!("There were {} feeds found with that identifier:", {
                matches_len
            });
            let iter = matches.iter().enumerate();
            for (i, feed) in iter {
                println!("\t-> {}) {}/{}", i + 1, &feed.category, &feed.title);
            }
            print!("Which one would you like to delete? ");
            io::stdout().flush()?;

            let mut choice = String::new();
            io::stdin().read_line(&mut choice)?;

            let normalized_input = choice.trim();

            match normalized_input.parse::<usize>() {
                Ok(ind) => {
                    if ind >= 1 && ind <= matches_len {
                        let title =
                            format!("{}/{}", &matches[ind - 1].category, &matches[ind - 1].title);

                        if confirm_delete(&title)? {
                            library
                                .delete_feed(&matches[ind - 1].slug, &matches[ind - 1].category)?;
                            info!("Feed deleted: {}", &matches[ind - 1].title);
                            println!("Feed deleted: {}", &matches[ind - 1].title);
                        } else {
                            info!("Feed was not deleted: {}", &title);
                            println!("Feed was not deleted: {}", &title);
                        }
                    } else {
                        info!("Invalid input received: {}", ind);
                        println!("Invalid input received: {}", ind);
                    }
                }
                Err(_) => {
                    info!("Invalid input received: {}", normalized_input);
                    println!("Invalid input received: {}", normalized_input);
                }
            }
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
                info!("Feed added: {}", feed.title);
                println!("Feed added: {}", feed.title);
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
