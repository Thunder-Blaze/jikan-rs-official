use serde::{Deserialize, Serialize};
use crate::utils::Images;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub mal_id: i32,
    pub url: String,
    pub images: Images,
    pub name: String,
    pub nicknames: Option<Vec<String>>,
    pub favorites: Option<i32>,
}