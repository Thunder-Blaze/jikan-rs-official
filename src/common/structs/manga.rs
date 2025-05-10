use serde::{Deserialize, Serialize};
use crate::utils::{Images, DateRange};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manga {
    pub mal_id: i32,
    pub url: String,
    pub images: Images,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub chapters: Option<i32>,
    pub volumes: Option<i32>,
    pub status: Option<String>,
    pub start_year: Option<u32>,
    pub score: Option<f32>,
    pub synopsis: Option<String>,
    pub published: Option<DateRange>,
}