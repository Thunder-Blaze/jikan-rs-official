use crate::{
    enums::forum::ForumFilter, response::Response, structs::{
        character::CharacterRole, 
        forum::{ForumTopic, NewsItem}, 
        manga::{Manga, MangaRelation, MangaStatistics, MoreInfo}, 
        reviews::Review, 
        users::UserUpdate,
        recommendation::RecommendationAlt
    }, utils::{ExternalEntry, Images}, JikanClient, JikanError
};

impl JikanClient {
    pub async fn get_manga(&self, id: i32) -> Result<Response<Manga>, JikanError> {
        self.get(&format!("/manga/{}", id)).await
    }

    pub async fn get_manga_full(&self, id: i32) -> Result<Response<Manga>, JikanError> {
        self.get(&format!("/manga/{}/full", id)).await
    }

    pub async fn get_manga_characters(&self, id: i32) -> Result<Vec<CharacterRole>, JikanError> {
        self.get(&format!("/manga/{}/characters", id)).await
    }

    pub async fn get_manga_news(
        &self,
        id: i32,
        page: Option<u32>,
    ) -> Result<Response<Vec<NewsItem>>, JikanError> {
        let mut path = format!("/manga/{}/news", id);
        
        if let Some(p) = page {
            path = format!("/manga/{}/news?page={}", id, p);
        }

        self.get(&path).await
    }

    pub async fn get_manga_forum(
        &self,
        id: i32,
        filter: Option<ForumFilter>,
    ) -> Result<Vec<ForumTopic>, JikanError> {
        let mut path = format!("/manga/{}/forum", id);

        if let Some(p) = filter {
            path = format!("/manga/{}/forum?filter={}", id, p.as_str());
        };

        self.get(&path).await
    }

    pub async fn get_manga_pictures(
        &self,
        id: i32,
    ) -> Result<Response<Vec<Images>>, JikanError> {
        self.get(&format!("/manga/{}/pictures", id)).await
    }

    pub async fn get_manga_statistics(
        &self,
        id: i32,
    ) -> Result<Response<MangaStatistics>, JikanError> {
        self.get(&format!("/manga/{}/statistics", id)).await
    }

    pub async fn get_manga_moreinfo(&self, id: i32) -> Result<Response<MoreInfo>, JikanError> {
        self.get(&format!("/manga/{}/moreinfo", id)).await
    }

    pub async fn get_manga_recommendations(
        &self,
        id: i32,
    ) -> Result<Response<Vec<RecommendationAlt>>, JikanError> {
        self.get(&format!("/manga/{}/recommendations", id)).await
    }

    pub async fn get_manga_userupdates(
        &self,
        id: i32,
        page: Option<u32>,
    ) -> Result<Response<Vec<UserUpdate>>, JikanError> {
        let mut path = format!("/manga/{}/userupdates", id);
        
        if let Some(p) = page {
            path = format!("/manga/{}/userupdates?page={}", id, p);
        }

        self.get(&path).await
    }

    pub async fn get_manga_reviews(
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

        self.get(&format!("/manga/{}/reviews{}", id, query)).await
    }

    pub async fn get_manga_relations(
        &self,
        id: i32,
    ) -> Result<Response<Vec<MangaRelation>>, JikanError> {
        self.get(&format!("/manga/{}/relations", id)).await
    }

    pub async fn get_manga_external(
        &self,
        id: i32,
    ) -> Result<Response<Vec<ExternalEntry>>, JikanError> {
        self.get(&format!("/manga/{}/external", id)).await
    }
}
