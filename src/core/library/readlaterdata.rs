use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ReadLaterData {
    pub read_later: Vec<String>,
}
