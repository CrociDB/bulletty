use reqwest::blocking::get;
use slug::slugify;

use crate::{feed::feedentry::FeedEntry, library::feeditem::FeedItem};

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

    feed.slug = slugify(&feed.title);

    Ok(feed)
}

pub fn get_feed_entries(feed: &FeedItem) -> color_eyre::Result<Vec<FeedEntry>> {
    let response = get(&feed.url)?.text()?;
    get_feed_entries_doc(feed, &response)
}

pub fn get_feed_entries_doc(feed: &FeedItem, doctxt: &str) -> color_eyre::Result<Vec<FeedEntry>> {
    let doc = roxmltree::Document::parse(doctxt)?;

    println!("Starting to parse document...");

    let mut feed_tag = doc.root();
    if feed_tag.tag_name().name() == "rss" {
        feed_tag = feed_tag
            .descendants()
            .find(|t| t.tag_name().name() == "channel")
            .unwrap();
    }

    let mut feedentries = Vec::<FeedEntry>::new();

    for entry in feed_tag
        .descendants()
        .filter(|t| t.tag_name().name() == "item" || t.tag_name().name() == "entry")
    {
        let fe = FeedEntry {
            title: entry
                .descendants()
                .find(|t| t.tag_name().name() == "title")
                .and_then(|t| t.text())
                .unwrap_or("NOTITLE")
                .to_string(),

            // TODO: find author
            author: feed.author.clone(),

            url: entry
                .descendants()
                .find(|t| t.tag_name().name() == "id" || t.tag_name().name() == "link")
                .and_then(|t| t.text())
                .unwrap_or("NOURL")
                .to_string(),

            text: entry
                .descendants()
                .find(|t| t.tag_name().name() == "summary" || t.tag_name().name() == "content")
                .and_then(|t| t.text())
                .unwrap_or("NOURL")
                .to_string(),
        };

        feedentries.push(fe);
    }

    Ok(feedentries)
}
