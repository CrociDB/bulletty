use serde::{Deserialize, Serialize};

use crate::core::library::readlaterentry::ReadLaterEntry;

#[derive(Default, Serialize, Deserialize)]
pub struct ReadLaterData {
    pub entries: Vec<ReadLaterEntry>,
}
