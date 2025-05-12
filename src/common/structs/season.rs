use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonInfo {
    pub year: u32,
    pub seasons: Vec<String>,
}
