use crate::library::feeditem::FeedItem;

#[derive(Clone)]
pub struct FeedCategory {
    pub title: String,
    pub feeds: Vec<FeedItem>,
}
