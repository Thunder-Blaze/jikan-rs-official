use crate::{
    structs::anime::Anime,
    structs::users::User,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reactions {
    pub overall: u32,
    pub nice: u32,
    pub love_it: u32,
    pub funny: u32,
    pub confusing: u32,
    pub informative: u32,
    pub well_written: u32,
    pub creative: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub mal_id: u32,
    pub url: String,
    pub r#type: String,
    pub reactions: Reactions,
    pub review: String,
    pub score: u32,
    pub entry: Anime,
    pub user: Option<User>,
    pub date: String,
    pub tags: Vec<String>,
    pub is_spoiler: bool,
    pub is_preliminary: bool,
    pub episodes_watched: Option<u32>,
}