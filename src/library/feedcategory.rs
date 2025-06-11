use crate::library::feeditem::FeedItem;

#[derive(Default)]
pub struct FeedCategoryState {
    expanded: bool,
}

pub struct FeedCategory {
    pub title: String,
    pub feeds: Vec<FeedItem>,
    pub state: FeedCategoryState,
}

impl FeedCategory {
    pub fn new() -> FeedCategory {
        FeedCategory {
            title: String::from("Category"),
            feeds: Vec::new(),
            state: FeedCategoryState { expanded: false },
        }
    }
}
