use color_eyre::{Result, eyre};
use roxmltree::Node;

use crate::core::library::feedcategory::FeedCategory;

pub struct OpmlFeed {
    pub url: String,
    pub category: Option<String>,
}

pub fn get_opml_feeds(filename: &str) -> Result<Vec<OpmlFeed>> {
    let doc = std::fs::read_to_string(filename)?;
    let doc = roxmltree::Document::parse(&doc)?;

    let body = doc.descendants().find(|n| n.has_tag_name("body"));
    if body.is_none() {
        return Err(eyre::eyre!("No body found in {:?}", filename));
    }

    let outlines: Vec<Node> = body
        .unwrap()
        .children()
        .filter(|n| n.is_element() && n.has_tag_name("outline"))
        .collect();

    let mut opml_feeds = Vec::<OpmlFeed>::new();

    for o in outlines.iter() {
        if o.has_attribute("xmlUrl") {
            if let Ok(feed) = get_opml_feed(o, None) {
                opml_feeds.push(feed);
            }
        } else {
            let title = o.attribute("title").unwrap_or("");
            let feeds: Vec<OpmlFeed> = o
                .children()
                .map(|c| get_opml_feed(&c, Some(title.to_string())))
                .filter_map(Result::ok)
                .collect();

            opml_feeds.extend(feeds);
        }
    }

    Ok(opml_feeds)
}

fn get_opml_feed(node: &Node, category: Option<String>) -> Result<OpmlFeed> {
    if let Some(xml_url) = node.attribute("xmlUrl") {
        Ok(OpmlFeed {
            url: xml_url.to_string(),
            category,
        })
    } else {
        Err(eyre::eyre!("No xml attribute found in element"))
    }
}

pub fn save_opml(categories: &[FeedCategory], filename: &str) -> Result<()> {
    let mut text_categories = String::new();
    for category in categories.iter() {
        let mut text_feeds = String::new();
        for feed in category.feeds.iter() {
            let title = html_escape::encode_text(&feed.title);
            let description = html_escape::encode_text(&feed.description);

            text_feeds.push_str(&format!("\n            <outline text={:?} title={:?} description={:?} xmlUrl={:?} type=\"rss\" />", title, title, description, feed.feed_url));
        }

        let title = html_escape::encode_text(&category.title);

        text_categories.push_str(&format!(
            "\n        <outline text={:?} title={:?}>{}\n        </outline>",
            title, title, text_feeds
        ));
    }

    let opml = format!(
        r#"<?xml version='1.0' encoding='UTF-8' ?>
<opml version="1.0">
    <head>
        <title>Generated from bulletty</title>
        <url>https://github.com/CrociDB/bulletty</url>
    </head>
    <body>{text_categories}
    </body>
</opml>
"#
    );

    std::fs::write(filename, opml)?;

    Ok(())
}
