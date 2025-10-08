use color_eyre::Result;

use crate::core::library::feedcategory::FeedCategory;

pub fn save_opml(categories: &[FeedCategory], filename: &str) -> Result<()> {
    let mut text_categories = String::new();
    for category in categories.iter() {
        let mut text_feeds = String::new();
        for feed in category.feeds.iter() {
            text_feeds.push_str(&format!("\n            <outline text={:?} title={:?} description={:?} xmlUrl={:?} type=\"rss\" />", feed.title, feed.title, feed.description, feed.feed_url));
        }

        text_categories.push_str(&format!(
            "\n        <outline text={:?} title={:?}>{}\n        </outline>",
            category.title, category.title, text_feeds
        ));
    }

    let opml = format!(
        r#"<?xml version='1.0' encoding='UTF-8' ?>
<opml version="1.0">
    <head>
        <title>Generated from bulletty</title>
        <url>https://github.com/CrociDB/bulletty</url>
    </head>
    <body>{}
    </body>
</opml>
"#,
        text_categories
    );

    std::fs::write(filename, opml)?;

    Ok(())
}
