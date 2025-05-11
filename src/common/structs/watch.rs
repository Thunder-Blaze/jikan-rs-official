use serde::{Deserialize, Serialize};
use crate::structs::anime::Anime;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrailerImages {
    pub image_url: Option<String>,
    pub small_image_url: Option<String>,
    pub medium_image_url: Option<String>,
    pub large_image_url: Option<String>,
    pub maximum_image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trailer {
    pub youtube_id: Option<String>,
    pub url: Option<String>,
    pub embed_url: Option<String>,
    pub images: Option<TrailerImages>,
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
    pub premium: Option<bool>,
    pub episode: Option<String>,
    pub aired: Option<String>,
    pub score: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchEpisodeEntry {
    pub entry: Anime,
    pub episodes: Vec<Episode>,
    pub region_locked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchPromoEntry {
    pub entry: Anime,
    pub trailer: Trailer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Videos {
    pub promo: Vec<PromoVideo>,
    pub episodes: Vec<Episode>,
}