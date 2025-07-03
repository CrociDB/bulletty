use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU16, Ordering::Relaxed},
    },
    thread::{self, JoinHandle},
};

use tracing::{error, info};

use crate::library::{feedcategory::FeedCategory, feedlibrary::FeedLibrary};

pub struct Updater {
    pub last_completed: Arc<Mutex<String>>,
    pub total_completed: Arc<AtomicU16>,
    pub finished: Arc<AtomicBool>,

    pub _stop_signal: Arc<AtomicBool>,
    _thread: Option<JoinHandle<()>>,
}

impl Updater {
    pub fn new(feedcategories: Vec<FeedCategory>) -> Self {
        let completed = Arc::new(Mutex::new(String::from("Working...")));
        let stop_signal = Arc::new(AtomicBool::new(false));
        let finished = Arc::new(AtomicBool::new(false));
        let total_completed = Arc::new(AtomicU16::new(0));

        let completed_clone = Arc::clone(&completed);
        let stop_signal_clone = Arc::clone(&stop_signal);
        let finished_clone = Arc::clone(&finished);
        let total_completed_clone = Arc::clone(&total_completed);

        let handle = Some(thread::spawn(move || {
            info!("Starting updater");
            let library = FeedLibrary::new();

            for category in feedcategories.iter() {
                for feed in category.feeds.iter() {
                    if stop_signal_clone.load(Relaxed) {
                        return;
                    }

                    if library
                        .data
                        .update_feed_entries(category, feed, None)
                        .is_err()
                    {
                        error!("Something happened when updating {}", &feed.title);
                        return;
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
            _stop_signal: stop_signal,
            _thread: handle,
            finished,
        }
    }
}
