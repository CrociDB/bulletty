use std::path::{Path, PathBuf};

use crate::defs;

pub struct Data {
    path: PathBuf,
}

impl Data {
    pub fn new(datapath: &Path) -> Data {
        load_or_create(datapath);
        Data {
            path: PathBuf::from(datapath),
        }
    }
}

pub fn load_or_create(path: &Path) {
    let datapath = Path::new(path);
    if !datapath.exists() {
        std::fs::create_dir_all(datapath).expect("Error: Failed to create datapath directory");
        std::fs::create_dir_all(datapath.join(defs::DATA_CATEGORIES_DIR)).expect("Error: Failed to create datapath directory");
    }
}
