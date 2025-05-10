use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Magazine {
    pub mal_id: i32,
    pub name: String,
    pub url: String,
    pub count: i32,
}