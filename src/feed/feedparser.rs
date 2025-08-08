use std::path::PathBuf;

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use html2md::parse_html;
use regex::Regex;
use reqwest::blocking::get;
use roxmltree::Node;
use slug::slugify;
use tracing::error;

use crate::{feed::feedentry::FeedEntry, library::feeditem::FeedItem};

pub fn get_feed(url: &str) -> color_eyre::Result<FeedItem> {
    let response = get(url)?.text()?;
    parse(&response, url)
}

fn parse(doc: &str, feed_url: &str) -> color_eyre::Result<FeedItem> {
    let mut feed = FeedItem::default();

    let doc = roxmltree::Document::parse(doc)?;
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
        .find(|t| t.tag_name().name() == "description" || t.tag_name().name() == "subtitle")
        .and_then(|t| t.text())
        .unwrap_or(&feed.title)
        .to_string();

    feed.url = feed_tag
        .descendants()
        .find(|t| t.tag_name().name() == "link")
        .and_then(|t| {
            if t.text().is_none() {
                t.attribute("href")
            } else {
                t.text()
            }
        })
        .unwrap_or(feed_url)
        .to_string();

    feed.feed_url = feed_url.to_string();

    if let Some(author_tag) = feed_tag
        .descendants()
        .find(|t| t.tag_name().name() == "author")
    {
        if let Some(nametag) = author_tag
            .descendants()
            .find(|t| t.tag_name().name() == "name")
            .and_then(|t| t.text())
        {
            feed.author = String::from(nametag);
        } else if let Some(text) = author_tag.text() {
            feed.author = String::from(text);
        } else {
            feed.author = "NOAUTHOR".to_string();
        }
    }

    feed.slug = slugify(&feed.title);

    feed.lastupdated = Utc::now();

    Ok(feed)
}

pub fn get_feed_entries(feed: &FeedItem) -> color_eyre::Result<Vec<FeedEntry>> {
    let response = get(&feed.feed_url)?.text()?;
    get_feed_entries_doc(&response)
}

pub fn get_feed_entries_doc(doctxt: &str) -> color_eyre::Result<Vec<FeedEntry>> {
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

        // date extraction
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

        // author extraction
        let entryauthor: String = if let Some(author_tag) = feed_tag
            .descendants()
            .find(|t| t.tag_name().name() == "author")
        {
            if let Some(nametag) = author_tag
                .descendants()
                .find(|t| t.tag_name().name() == "name")
                .and_then(|t| t.text())
            {
                String::from(nametag)
            } else if let Some(text) = author_tag.text() {
                String::from(text)
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        };

        // feed creation
        let fe = FeedEntry {
            title: entry
                .descendants()
                .find(|t| t.tag_name().name() == "title")
                .and_then(|t| t.text())
                .unwrap_or("NOTITLE")
                .to_string(),
            author: entryauthor,
            url: entry
                .descendants()
                .find(|t| t.tag_name().name() == "id" || t.tag_name().name() == "link")
                .and_then(|t| t.text())
                .unwrap_or("NOURL")
                .to_string(),
            text: content,
            date: parse_date(&datestr)
                .map_err(|err| error!("{:?}", err))
                .unwrap_or_default(),
            description: desc,
            lastupdated: Utc::now(),
            seen: false,
            filepath: PathBuf::default(),
        };

        feedentries.push(fe);
    }

    Ok(feedentries)
}

fn parse_date(date_str: &str) -> color_eyre::Result<DateTime<Utc>> {
    // Attempt to parse as RFC3339 (e.g., "2024-01-01T12:00:00Z" or "2024-01-01T12:00:00+01:00")
    if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) {
        return Ok(dt.with_timezone(&Utc));
    }

    // Attempt to parse as RFC2822 (e.g., "Tue, 01 Jan 2024 12:00:00 +0000")
    if let Ok(dt) = DateTime::parse_from_rfc2822(date_str) {
        return Ok(dt.with_timezone(&Utc));
    }

    // Attempt to parse with the specific format "Tue, 01 Jan 2024 12:00:00 +0000"
    let format_with_offset = "%a, %d %b %Y %H:%M:%S %z";
    if let Ok(dt) = DateTime::parse_from_str(date_str, format_with_offset) {
        return Ok(dt.with_timezone(&Utc));
    }

    // Attempt to parse a NaiveDateTime with no offset (e.g., "2024-01-01 12:00:00")
    let format_naive_datetime = "%Y-%m-%d %H:%M:%S";
    if let Ok(naive) = NaiveDateTime::parse_from_str(date_str, format_naive_datetime) {
        return Ok(DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc));
    }

    // Attempt to parse a NaiveDate (e.g., "2024-01-01") and set time to midnight UTC
    let format_naive_date = "%Y-%m-%d";
    if let Ok(naive_date) = NaiveDate::parse_from_str(date_str, format_naive_date) {
        if let Some(naive_datetime) = naive_date.and_hms_opt(0, 0, 0) {
            return Ok(DateTime::<Utc>::from_naive_utc_and_offset(
                naive_datetime,
                Utc,
            ));
        }
    }

    Err(color_eyre::eyre::eyre!(
        "Couldn't parse date: {:?}",
        date_str
    ))
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
        r"#+\s*",             // headings
        r"!\[(.*?)\]\(.*?\)", // images
        r"\[(.*?)\]\(.*?\)",  // links
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

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;

    #[test]
    fn test_strip_markdown_tags() {
        let input = "**bold** *italic* `code` ~~strike~~ [link](url) ![image](url) # heading > blockquote\n---\n";
        let expected = "bold italic code strike link image heading blockquote\n\n";
        assert_eq!(strip_markdown_tags(input), expected);
    }

    #[test]
    fn test_parse_date_various_formats() {
        let datetime_strings = [
            "2024-01-01T12:00:00Z",      // RFC3339 UTC
            "2024-01-01T13:00:00+01:00", // RFC3339 with offset
            "2024-02-29 09:00:00",       // Naive datetime
            "2023-11-20",                // Naive date
            "Invalid Date String",       // Invalid format
        ];

        let expected = [
            Some(
                DateTime::parse_from_rfc3339("2024-01-01T12:00:00+00:00")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            Some(
                DateTime::parse_from_rfc3339("2024-01-01T12:00:00+00:00")
                    .unwrap()
                    .with_timezone(&Utc),
            ), // 13:00+01:00 == 12:00Z
            Some(Utc.with_ymd_and_hms(2024, 2, 29, 9, 0, 0).unwrap()),
            Some(Utc.with_ymd_and_hms(2023, 11, 20, 0, 0, 0).unwrap()),
            None,
        ];

        for (input, expected_str) in datetime_strings.iter().zip(expected.iter()) {
            let result = parse_date(input);
            match expected_str {
                Some(exp) => match result {
                    Ok(ref dt) => assert_eq!(dt, exp, "Failed on input: {}", input),
                    Err(_) => panic!("Expected Ok for input: {}", input),
                },
                None => assert!(result.is_err(), "Expected error for input: {}", input),
            }
        }
    }

    #[test]
    fn parses_rss2_channel_fields() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0">
  <channel>
    <title>Example RSS</title>
    <link>https://example.com/</link>
    <description>RSS description</description>
    <author>Alice</author>
    <item>
      <title>Item 1</title>
      <link>https://example.com/item1</link>
      <description>Item 1 description</description>
      <author>alice@example.com (Alice)</author>
    </item>
  </channel>
</rss>"#;

        let feed = parse(xml, "NOURL").expect("failed to parse RSS 2.0");
        assert_eq!(feed.title, "Example RSS");
        assert_eq!(feed.description, "RSS description");
        assert_eq!(feed.url, "https://example.com/");
        assert!(feed.author.contains("Alice"));
    }

    #[test]
    fn parses_atom_feed_fields() {
        let xml = r#"<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>Example Atom</title>
  <subtitle>Atom description</subtitle>
  <link href="https://example.org/"/>
  <author>
    <name>Bob</name>
  </author>
  <id>urn:uuid:60a76c80-d399-11d9-b93C-0003939e0af6</id>
  <updated>2003-12-13T18:30:02Z</updated>
</feed>"#;

        let feed = parse(xml, "NOURL").expect("failed to parse Atom");
        assert_eq!(feed.title, "Example Atom");
        assert_eq!(feed.description, "Atom description");
        assert_eq!(feed.url, "https://example.org/");
        assert_eq!(feed.author, "Bob");
    }

    #[test]
    fn rss_missing_link_uses_default_url() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0">
  <channel>
    <title>No Link RSS</title>
    <description>No link here</description>
    <author>Carol</author>
  </channel>
</rss>"#;

        let feed = parse(xml, "NOURL").expect("failed to parse RSS without link");
        assert_eq!(feed.title, "No Link RSS");
        assert_eq!(feed.description, "No link here");
        assert_eq!(feed.url, "NOURL");
        assert!(feed.author.contains("Carol"));
    }
} 

