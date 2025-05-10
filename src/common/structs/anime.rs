use serde::{Deserialize, Serialize};
use crate::utils::{Images, DateRange};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anime {
    pub mal_id: i32,
    pub url: String,
    pub images: Images,
    pub title: String,
    pub start_year: Option<u32>,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub episodes: Option<i32>,
    pub status: Option<String>,
    pub score: Option<f32>,
    pub synopsis: Option<String>,
    pub aired: Option<DateRange>,
}