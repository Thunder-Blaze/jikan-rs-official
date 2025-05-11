use serde::{Deserialize, Serialize};
use crate::{
    utils::{Images, DateRange, Title, Score},
    response::MalCommonResponse,
    structs::{
        people::Person,
        character::Character,
        forum::ForumTopic,
    }
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anime {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub title: String,
    pub start_year: Option<u32>,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub episodes: Option<u32>,
    pub status: Option<String>,
    pub score: Option<f32>,
    pub synopsis: Option<String>,
    pub aired: Option<DateRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Broadcast {
    pub day: Option<String>,
    pub time: Option<String>,
    pub timezone: Option<String>,
    pub string: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trailer {
    pub youtube_id: Option<String>,
    pub url: Option<String>,
    pub embed_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeExtended {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub trailer: Option<Trailer>,
    pub approved: Option<bool>,
    pub titles: Option<Vec<Title>>,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub title_synonyms: Option<Vec<String>>,
    pub r#type: Option<String>,
    pub source: Option<String>,
    pub episodes: Option<u32>,
    pub status: Option<String>,
    pub airing: Option<bool>,
    pub aired: Option<DateRange>,
    pub duration: Option<String>,
    pub rating: Option<String>,
    pub score: Option<f32>,
    pub scored_by: Option<u32>,
    pub rank: Option<u32>,
    pub popularity: Option<u32>,
    pub members: Option<u32>,
    pub favorites: Option<u32>,
    pub synopsis: Option<String>,
    pub background: Option<String>,
    pub season: Option<String>,
    pub year: Option<u32>,
    pub broadcast: Option<Broadcast>,
    pub producers: Option<Vec<MalCommonResponse>>,
    pub licensors: Option<Vec<MalCommonResponse>>,
    pub studios: Option<Vec<MalCommonResponse>>,
    pub genres: Option<Vec<MalCommonResponse>>,
    pub explicit_genres: Option<Vec<MalCommonResponse>>,
    pub themes: Option<Vec<MalCommonResponse>>,
    pub demographics: Option<Vec<MalCommonResponse>>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromoVideo {
    pub title: String,
    pub trailer: Trailer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub mal_id: i32,
    pub url: Option<String>,
    pub title: String,
    pub episode: Option<String>,
    pub aired: Option<String>,
    pub score: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Videos {
    pub promo: Vec<PromoVideo>,
    pub episodes: Vec<Episode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeCharacter {
    pub character: Character,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffMember {
    pub person: Person,
    pub positions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeThemes {
    pub openings: Vec<String>,
    pub endings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeForum {
    pub data: Vec<ForumTopic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Picture {
    pub images: Images,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoreInfo {
    pub moreinfo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeStatistics {
    pub watching: i32,
    pub completed: i32,
    pub on_hold: i32,
    pub dropped: i32,
    pub plan_to_watch: i32,
    pub total: i32,
    pub scores: Vec<Score>,
}