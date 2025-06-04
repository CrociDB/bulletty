use color_eyre::eyre::eyre;

use crate::library::feeditem::FeedItem;

pub fn parse(url: &str) -> color_eyre::Result<FeedItem> {
    Err(eyre!("Couldn't add URL"))
}
