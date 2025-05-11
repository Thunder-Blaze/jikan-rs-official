use crate::{
    JikanClient, JikanError,
    response::Response,
    structs::watch::{WatchEpisodeEntry, WatchPromoEntry},
};

impl JikanClient {
    /// Returns Recently Added Episodes
    pub async fn get_watch_recent_episodes(&self) -> Result<Response<Vec<WatchEpisodeEntry>>, JikanError> {
        self.get("/watch/episodes").await
    }

    /// Returns Popular Episodes
    pub async fn get_watch_popular_episodes(&self) -> Result<Response<Vec<WatchEpisodeEntry>>, JikanError> {
        self.get("/watch/episodes/popular").await
    }

    /// Returns Recently Added Promotional Videos
    pub async fn get_watch_recent_promos(
        &self,
        page: Option<u32>,
    ) -> Result<Response<Vec<WatchPromoEntry>>, JikanError> {
        let mut path = "/watch/promos".to_string();

        if let Some(p) = page {
            path = format!("/watch/promos?page={}", p);
        }
        
        self.get(&path).await
    }

    /// Returns Popular Promotional Videos
    pub async fn get_watch_popular_promos(&self) -> Result<Response<Vec<WatchPromoEntry>>, JikanError> {
        self.get("/watch/promos/popular").await
    }
}
