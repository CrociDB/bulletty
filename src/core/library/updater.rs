use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU16, Ordering::Relaxed},
    },
    thread::{self, JoinHandle},
};

use tracing::{error, info};

use crate::core::library::{feedcategory::FeedCategory, feedlibrary::FeedLibrary};

pub struct Updater {
    pub last_completed: Arc<Mutex<String>>,
    pub total_completed: Arc<AtomicU16>,
    pub finished: Arc<AtomicBool>,

    _thread: Option<JoinHandle<()>>,
}

impl Updater {
    pub fn new(feedcategories: Vec<FeedCategory>) -> Self {
        let completed = Arc::new(Mutex::new(String::from("Working...")));
        let finished = Arc::new(AtomicBool::new(false));
        let total_completed = Arc::new(AtomicU16::new(0));

        let completed_clone = Arc::clone(&completed);
        let finished_clone = Arc::clone(&finished);
        let total_completed_clone = Arc::clone(&total_completed);

        let handle = Some(thread::spawn(move || {
            info!("Starting updater");
            let library = FeedLibrary::new();

            for category in feedcategories.iter() {
                for feed in category.feeds.iter() {
                    if let Err(e) = library
                        .data
                        .update_feed_entries(&category.title, feed, None)
                    {
                        error!("Something happened when updating {}: {:?}", &feed.title, e);
                        break;
                    }

                    info!("Updated {}", &feed.title);

                    total_completed_clone.fetch_add(1, Relaxed);
                    *completed_clone.lock().unwrap() = feed.title.clone();
                }
            }

            finished_clone.store(true, Relaxed);
        }));

        Self {
            last_completed: completed,
            total_completed,
            _thread: handle,
            finished,
        }
    }
}
