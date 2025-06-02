use crate::library::feeditem::FeedItem;

pub struct FeedCategoryState {
    expanded: bool,
}

// Category
pub struct FeedCategory {
    pub title: String,
    pub feeds: Vec<FeedItem>,
    pub state: FeedCategoryState,
}

impl FeedCategory {
    pub fn new() -> FeedCategory {
        FeedCategory {title: String::from("Category"), feeds: Vec::new(), state: FeedCategoryState{ expanded: false } }
    }
}

// Library
pub struct FeedLibrary {
    pub feedcategories: Vec<FeedCategory>,
}

impl FeedLibrary {
    pub fn new() -> FeedLibrary {
        FeedLibrary {
            feedcategories: vec![
                FeedCategory::new(),
                FeedCategory::new(),
                FeedCategory::new(),
            ],
        }
    }
}
