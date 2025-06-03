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
        FeedCategory {
            title: String::from("Category"),
            feeds: Vec::new(),
            state: FeedCategoryState { expanded: false },
        }
    }
}

// Library
pub struct FeedLibrary {
    pub feedcategories: Vec<FeedCategory>,
    pub currentselection: usize,
    pub totalitems: usize,
}

impl FeedLibrary {
    pub fn new() -> FeedLibrary {
        FeedLibrary {
            currentselection: 0,
            totalitems: 0,
            feedcategories: vec![
                FeedCategory::new(),
                FeedCategory::new(),
                FeedCategory::new(),
            ],
        }
    }

    pub fn get_list_data(&self) -> Vec<String> {
        let mut items = Vec::<String>::new();
        for (i, item) in self.feedcategories.iter().enumerate() {
            let title = format!(" > {}", item.title);
            items.push(title);
        }

        items
    }

    pub fn selection_up(&mut self) {
        if self.currentselection > 0 {
            self.currentselection -= 1;
        }
    }

    pub fn selection_down(&mut self) {
        self.currentselection =
            std::cmp::min(self.currentselection + 1, self.count_total_items() - 1);
    }

    fn count_total_items(&self) -> usize {
        self.feedcategories.len()
    }
}
