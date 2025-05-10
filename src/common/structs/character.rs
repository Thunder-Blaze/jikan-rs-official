use serde::{Deserialize, Serialize};
use crate::utils::Images;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub name: String,
    pub name_kanji: Option<String>,
    pub nicknames: Option<Vec<String>>,
    pub favorites: Option<u32>,
    pub about: Option<String>,
}