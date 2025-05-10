use crate::{JikanClient, JikanError, utils::Pagination};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchEpisodesResponse {
    pub data: Vec<WatchEpisodeEntry>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchEpisodeEntry {
    pub entry: AnimeEpisodeEntry,
    pub episodes: Vec<EpisodeDetails>,
    pub region_locked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeEpisodeEntry {
    pub mal_id: i32,
    pub url: String,
    pub images: crate::utils::Images,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeDetails {
    pub mal_id: i32,
    pub url: String,
    pub title: String,
    pub premium: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchPromosResponse {
    pub data: Vec<WatchPromoEntry>,
    pub pagination: Pagination,
    #[serde(default = "default_title")]
    pub title: String,
}

fn default_title() -> String {
    "Promotional Videos".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchPromoEntry {
    pub entry: AnimeEpisodeEntry,
    #[serde(default)]
    pub trailer: TrailerData,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrailerData {
    #[serde(default)]
    pub youtube_id: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub embed_url: Option<String>,
    #[serde(default)]
    pub images: Option<TrailerImages>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrailerImages {
    #[serde(default)]
    pub image_url: Option<String>,
    #[serde(default)]
    pub small_image_url: Option<String>,
    #[serde(default)]
    pub medium_image_url: Option<String>,
    #[serde(default)]
    pub large_image_url: Option<String>,
    #[serde(default)]
    pub maximum_image_url: Option<String>,
}

impl JikanClient {
    /// Returns Recently Added Episodes
    pub async fn get_watch_recent_episodes(&self) -> Result<WatchEpisodesResponse, JikanError> {
        self.get("/watch/episodes").await
    }

    /// Returns Popular Episodes
    pub async fn get_watch_popular_episodes(&self) -> Result<WatchEpisodesResponse, JikanError> {
        self.get("/watch/episodes/popular").await
    }

    /// Returns Recently Added Promotional Videos
    pub async fn get_watch_recent_promos(
        &self,
        page: Option<u32>,
    ) -> Result<WatchPromosResponse, JikanError> {
        let path = match page {
            Some(p) => format!("/watch/promos?page={}", p),
            None => "/watch/promos".to_string(),
        };
        self.get(&path).await
    }

    /// Returns Popular Promotional Videos
    pub async fn get_watch_popular_promos(&self) -> Result<WatchPromosResponse, JikanError> {
        self.get("/watch/promos/popular").await
    }
}
