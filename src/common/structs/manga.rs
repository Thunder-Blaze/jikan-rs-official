use serde::{Deserialize, Serialize};
use crate::{
    utils::{Images, DateRange, Title, Score},
    response::MalCommonResponse,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manga {
    pub mal_id: u32,
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


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaExtended {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub approved: bool,
    pub titles: Vec<Title>,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub r#type: String,
    pub chapters: Option<u32>,
    pub volumes: Option<u32>,
    pub status: String,
    pub publishing: bool,
    pub published: DateRange,
    pub score: Option<f32>,
    pub scored_by: Option<u32>,
    pub rank: Option<u32>,
    pub popularity: Option<u32>,
    pub members: Option<u32>,
    pub favorites: Option<u32>,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub authors: Vec<MalCommonResponse>,
    pub serializations: Vec<MalCommonResponse>,
    pub genres: Vec<MalCommonResponse>,
    pub explicit_genres: Vec<MalCommonResponse>,
    pub themes: Vec<MalCommonResponse>,
    pub demographics: Vec<MalCommonResponse>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaRelation {
    pub relation: String,
    pub entry: Vec<MalCommonResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoreInfo {
    pub moreinfo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaStatistics {
    pub reading: i32,
    pub completed: i32,
    pub on_hold: i32,
    pub dropped: i32,
    pub plan_to_read: i32,
    pub total: i32,
    pub scores: Vec<Score>,
}