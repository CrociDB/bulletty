use crate::library::feeditem::FeedItem;

pub struct FeedCategory {
    pub title: String,
    pub feeds: Vec<FeedItem>,
}
