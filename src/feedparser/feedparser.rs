use color_eyre::eyre::eyre;

use reqwest::blocking::get;

use crate::library::feeditem::FeedItem;

pub fn parse(url: &str) -> color_eyre::Result<FeedItem> {
    println!("Loading feed...");
    let response = get(url)?.text()?;
    println!("Parsing feed...");

    let mut feed = FeedItem::default();

    let doc = roxmltree::Document::parse(response.as_str())?;
    let mut feed_tag = doc.root();
    if feed_tag.tag_name().name() == "rss" {
        feed_tag = feed_tag
            .descendants()
            .find(|t| t.tag_name().name() == "channel")
            .unwrap();
    }

    feed.title = feed_tag
        .descendants()
        .find(|t| t.tag_name().name() == "title")
        .and_then(|t| t.text())
        .unwrap_or("")
        .to_string();

    feed.description = feed_tag
        .descendants()
        .find(|t| t.tag_name().name() == "description")
        .and_then(|t| t.text())
        .unwrap_or(&feed.title)
        .to_string();

    feed.url = feed_tag
        .descendants()
        .find(|t| t.tag_name().name() == "link")
        .and_then(|t| t.text())
        .unwrap_or(url)
        .to_string();

    if let Some(author_tag) = feed_tag
        .descendants()
        .find(|t| t.tag_name().name() == "author")
    {
        if let Some(text) = author_tag.text() {
            feed.author = String::from(text);
        } else {
            feed.author = author_tag
                .children()
                .find(|t| t.tag_name().name() == "name")
                .and_then(|t| t.text())
                .unwrap_or("NOAUTHOR")
                .to_string();
        }
    }

    Ok(feed)
}
