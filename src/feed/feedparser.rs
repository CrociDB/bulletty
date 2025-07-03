use chrono::{DateTime, Utc};
use html2md::parse_html;
use regex::Regex;
use reqwest::blocking::get;
use roxmltree::Node;
use slug::slugify;

use crate::{feed::feedentry::FeedEntry, library::feeditem::FeedItem};

pub fn parse(url: &str) -> color_eyre::Result<FeedItem> {
    let response = get(url)?.text()?;

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

    // TODO: find the proper url of the post
    // feed.url = feed_tag
    //     .descendants()
    //     .find(|t| t.tag_name().name() == "link")
    //     .and_then(|t| t.text())
    //     .unwrap_or(url)
    //     .to_string();
    feed.url = String::from(url);

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
        let (desc, content) = get_description_content(&entry);

        let datestr = entry
            .descendants()
            .find(|t| {
                t.tag_name().name() == "published"
                    || t.tag_name().name() == "updated"
                    || t.tag_name().name() == "date"
                    || t.tag_name().name() == "pubDate"
            })
            .and_then(|t| t.text())
            .unwrap_or("1990-09-19")
            .to_string();

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

            text: content,
            date: parse_date(&datestr),
            description: desc,
        };

        feedentries.push(fe);
    }

    Ok(feedentries)
}

fn parse_date(date_str: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(date_str)
        .map(|dt| dt.with_timezone(&Utc))
        .or_else(|_| {
            chrono::NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S")
                .map(|naive| DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc))
        })
        .or_else(|_| {
            chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map(|naive_date| {
                let naive = naive_date.and_hms_opt(0, 0, 0).unwrap();
                DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc)
            })
        })
        .or_else(|_| {
            DateTime::parse_from_str(date_str, "%a, %d %b %Y %H:%M:%S %z")
                .map(|dt| dt.with_timezone(&Utc))
        })
        .unwrap_or_else(|_| {
            let fallback = chrono::NaiveDate::from_ymd_opt(1990, 9, 19)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap();
            DateTime::<Utc>::from_naive_utc_and_offset(fallback, Utc)
        })
}

fn get_description_content(entry: &Node) -> (String, String) {
    let content = entry
        .descendants()
        .find(|t| t.tag_name().name() == "content" || t.tag_name().name() == "encoded")
        .and_then(|t| t.text());

    let description = entry
        .descendants()
        .find(|t| t.tag_name().name() == "description" || t.tag_name().name() == "summary")
        .and_then(|t| t.text());

    let content_text = match content {
        Some(text) => parse_html(text),
        None => match description {
            Some(desc) => parse_html(desc),
            None => String::new(),
        },
    };

    let description_text = match description {
        Some(text) => parse_html(text)
            .replace("\n", "")
            .chars()
            .take(280)
            .collect::<String>(),
        None => content_text
            .replace("\n", "")
            .chars()
            .take(280)
            .collect::<String>(),
    };

    (strip_markdown_tags(&description_text), content_text)
}

fn strip_markdown_tags(input: &str) -> String {
    let patterns = [
        r"\*\*(.*?)\*\*",     // bold **
        r"\*(.*?)\*",         // italic *
        r"`(.*?)`",           // inline code
        r"~~(.*?)~~",         // strikethrough
        r"\[(.*?)\]\(.*?\)",  // links
        r"!\[(.*?)\]\(.*?\)", // images
        r"^#+\s*",            // headings
        r">+\s*",             // blockquotes
        r"[-*_=]{3,}",        // horizontal rules
        r"`{3}.*?`{3}",       // code blocks
    ];
    let mut result = input.to_string();
    for pat in patterns.iter() {
        let re = Regex::new(pat).unwrap();
        result = re.replace_all(&result, "$1").to_string();
    }
    result
}
