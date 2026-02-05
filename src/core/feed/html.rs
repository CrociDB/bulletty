use std::borrow::Cow;

use tl::{Bytes, Node};
use url::Url;

pub fn is_html(content: &str) -> bool {
    let trimmed = content.trim_start();
    trimmed.starts_with("<!DOCTYPE html")
        || trimmed.starts_with("<html")
        || trimmed.starts_with("<HTML")
}

pub fn extract_embedded_feed_urls(
    html: &str,
    url: &Url,
    maximum_feeds: usize,
) -> color_eyre::Result<Vec<String>> {
    let dom = tl::parse(html, tl::ParserOptions::default())?;
    let parser = dom.parser();

    let links = dom
        .query_selector("link[rel='alternate']")
        .into_iter()
        .flatten()
        .filter_map(|node_handle| {
            node_handle
                .get(parser)
                .and_then(Node::as_tag)
                .filter(|tag| get_attribute(tag, "type").is_some_and(is_feed))
                .and_then(|tag| get_attribute(tag, "href"))
                .and_then(|href| join(url, &href))
        })
        .take(maximum_feeds)
        .collect();

    Ok(links)
}

fn get_attribute<'a>(tag: &'a tl::HTMLTag<'_>, attribute: &'a str) -> Option<Cow<'a, str>> {
    tag.attributes()
        .get(attribute)
        .flatten()
        .map(Bytes::as_utf8_str)
}

fn is_feed(link_type: Cow<'_, str>) -> bool {
    let link_type = link_type.to_lowercase();
    link_type.contains("atom") || link_type.contains("rss")
}

fn join(url: &Url, href: &str) -> Option<String> {
    url.join(href).map(|url| url.to_string()).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_absolute_url() {
        let html = r#"<!DOCTYPE html>
<html>
<head>
<title>My Blog</title>
<link rel="alternate" type="application/rss+xml" href="https://example.com/feed.rss" />
</head>
<body>
<h1>Welcome</h1>
</body>
</html>"#;

        let feeds =
            extract_embedded_feed_urls(html, &Url::parse("https://example.com/").unwrap(), 10)
                .unwrap();
        assert_eq!(feeds, vec!["https://example.com/feed.rss"]);
    }

    #[test]
    fn extract_relative_url() {
        let html = r#"<!DOCTYPE html>
<html>
<head>
<title>My Blog</title>
<link rel="alternate" type="application/atom+xml" href="/feed.atom" />
</head>
<body>
<h1>Welcome</h1>
</body>
</html>"#;

        let feeds =
            extract_embedded_feed_urls(html, &Url::parse("https://example.com/blog/").unwrap(), 10)
                .unwrap();
        assert_eq!(feeds, vec!["https://example.com/feed.atom"]);
    }

    #[test]
    fn extract_mixed_feed_urls() {
        let html = r#"<!DOCTYPE html>
<html>
<head>
<title>Multi-feed Site</title>
<link rel="alternate" type="application/rss+xml" href="https://example.com/rss" />
<link rel="alternate" type="application/atom+xml" href="https://example.com/atom" />
</head>
<body>
<h1>Welcome</h1>
</body>
</html>"#;

        let feeds =
            extract_embedded_feed_urls(html, &Url::parse("https://example.com/").unwrap(), 10)
                .unwrap();
        assert_eq!(
            feeds,
            vec!["https://example.com/rss", "https://example.com/atom"]
        );
    }

    #[test]
    fn extract_limited_urls() {
        let html = r#"<!DOCTYPE html>
<html>
<head>
<title>Multi-feed Site</title>
<link rel="alternate" type="application/rss+xml" href="https://example.com/rss1" />
<link rel="alternate" type="application/rss+xml" href="https://example.com/rss2" />
<link rel="alternate" type="application/rss+xml" href="https://example.com/rss3" />
</head>
<body>
<h1>Welcome</h1>
</body>
</html>"#;

        let feeds =
            extract_embedded_feed_urls(html, &Url::parse("https://example.com/").unwrap(), 2)
                .unwrap();
        assert_eq!(
            feeds,
            vec!["https://example.com/rss1", "https://example.com/rss2"]
        );
    }

    #[test]
    fn extract_no_urls() {
        let html = r#"<!DOCTYPE html>
<html>
<head>
<title>No Feed Site</title>
</head>
<body>
<h1>Welcome</h1>
</body>
</html>"#;

        let feeds =
            extract_embedded_feed_urls(html, &Url::parse("https://example.com/").unwrap(), 10)
                .unwrap();
        assert!(feeds.is_empty());
    }

    #[test]
    fn html_doctype_is_html() {
        assert!(is_html("<!DOCTYPE html><html></html>"));
    }

    #[test]
    fn html_doctype_with_leading_whitespace_is_html() {
        assert!(is_html("  \n  <!DOCTYPE html><html></html>"));
    }

    #[test]
    fn html_tag_is_html() {
        assert!(is_html("<html><head></head></html>"));
    }

    #[test]
    fn rss_is_not_html() {
        assert!(!is_html("<?xml version=\"1.0\"?><rss></rss>"));
    }

    #[test]
    fn atom_is_not_html() {
        assert!(!is_html("<?xml version=\"1.0\"?><feed></feed>"));
    }
}
