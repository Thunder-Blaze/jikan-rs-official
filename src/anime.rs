// anime.rs
use crate::{
    utils::{ExternalEntry, Images},
    enums::forum::ForumFilter,
    response::Response, structs::{
        anime::{
            Anime, AnimeForum, AnimeStatistics, AnimeThemes, MoreInfo, StaffMember
        },
        watch::{Episode, Videos},
        forum::NewsItem,
        reviews::Review,
        character::CharacterRole,
        users::UserUpdate,
        recommendation::RecommendationAlt
    }, JikanClient, JikanError
};

impl JikanClient {
    pub async fn get_anime(&self, id: i32) -> Result<Response<Anime>, JikanError> {
        self.get(&format!("/anime/{}", id)).await
    }

    pub async fn get_anime_full(&self, id: i32) -> Result<Response<Anime>, JikanError> {
        self.get(&format!("/anime/{}/full", id)).await
    }

    pub async fn get_anime_characters(&self, id: i32) -> Result<Response<Vec<CharacterRole>>, JikanError> {
        self.get(&format!("/anime/{}/characters", id)).await
    }

    pub async fn get_anime_staff(&self, id: i32) -> Result<Response<Vec<StaffMember>>, JikanError> {
        self.get(&format!("/anime/{}/staff", id)).await
    }

    pub async fn get_anime_episodes(
        &self,
        id: i32,
        page: Option<u32>,
    ) -> Result<Response<Vec<Episode>>, JikanError> {
        let mut path = format!("/anime/{}/episodes", id);
        
        if let Some(p) = page {
            path = format!("/anime/{}/episodes?page={}", id, p);
        };

        self.get(&path).await
    }

    pub async fn get_anime_episode(
        &self,
        id: i32,
        episode: i32,
    ) -> Result<Response<Episode>, JikanError> {
        self.get(&format!("/anime/{}/episodes/{}", id, episode))
            .await
    }

    pub async fn get_anime_videos(&self, id: i32) -> Result<Response<Videos>, JikanError> {
        self.get(&format!("/anime/{}/videos", id)).await
    }

    pub async fn get_anime_statistics(
        &self,
        id: i32,
    ) -> Result<Response<AnimeStatistics>, JikanError> {
        self.get(&format!("/anime/{}/statistics", id)).await
    }

    pub async fn get_anime_themes(
        &self,
        id: i32,
    ) -> Result<Response<AnimeThemes>, JikanError> {
        self.get(&format!("/anime/{}/themes", id)).await
    }

    pub async fn get_anime_external(
        &self,
        id: i32,
    ) -> Result<Response<Vec<ExternalEntry>>, JikanError> {
        self.get(&format!("/anime/{}/external", id)).await
    }

    pub async fn get_anime_streaming(
        &self,
        id: i32,
    ) -> Result<Response<Vec<ExternalEntry>>, JikanError> {
        self.get(&format!("/anime/{}/streaming", id)).await
    }

    pub async fn get_anime_news(
        &self,
        id: i32,
        page: Option<u32>,
    ) -> Result<Response<Vec<NewsItem>>, JikanError> {
        let mut path = format!("/anime/{}/news", id);
        
        if let Some(p) = page {
            path = format!("/anime/{}/news?page={}", id, p);
        }

        self.get(&path).await
    }

    pub async fn get_anime_forum(
        &self,
        id: i32,
        filter: Option<ForumFilter>,
    ) -> Result<AnimeForum, JikanError> {
        let mut path = format!("/anime/{}/forum", id);
        
        if let Some(p) = filter {
            path = format!("/anime/{}/forum?filter={}", id, p.as_str());
        }
        
        self.get(&path).await
    }

    pub async fn get_anime_videos_episodes(
        &self,
        id: i32,
        page: Option<u32>,
    ) -> Result<Response<Vec<Episode>>, JikanError> {
        let mut path = format!("/anime/{}/videos/episodes", id);

        if let Some(p) = page {
            path = format!("/anime/{}/videos/episodes?page={}", id, p);
        }

        self.get(&path).await
    }

    pub async fn get_anime_pictures(
        &self,
        id: i32,
    ) -> Result<Response<Vec<Images>>, JikanError> {
        self.get(&format!("/anime/{}/pictures", id)).await
    }

    pub async fn get_anime_moreinfo(&self, id: i32) -> Result<Response<MoreInfo>, JikanError> {
        self.get(&format!("/anime/{}/moreinfo", id)).await
    }

    pub async fn get_anime_recommendations(
        &self,
        id: i32,
    ) -> Result<Response<Vec<RecommendationAlt>>, JikanError> {
        self.get(&format!("/anime/{}/recommendations", id)).await
    }

    pub async fn get_anime_userupdates(
        &self,
        id: i32,
        page: Option<u32>,
    ) -> Result<Response<Vec<UserUpdate>>, JikanError> {
        let mut path = format!("/anime/{}/userupdates", id);

        if let Some(p) = page {
            path = format!("/anime/{}/userupdates?page={}", id, p);
        }

        self.get(&path).await
    }

    pub async fn get_anime_reviews(
        &self,
        id: i32,
        page: Option<u32>,
        preliminary: Option<bool>,
        spoilers: Option<bool>,
    ) -> Result<Response<Vec<Review>>, JikanError> {
        let mut params = Vec::new();

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }
        if let Some(pr) = preliminary {
            params.push(format!("preliminary={}", pr));
        }
        if let Some(sp) = spoilers {
            params.push(format!("spoilers={}", sp));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/anime/{}/reviews{}", id, query)).await
    }
}
