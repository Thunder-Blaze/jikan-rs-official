use crate::{
    JikanClient, JikanError,
    utils::{Pagination, ExternalEntry},
    enums::forum::ForumFilter,
    structs::{
        character::Character,
        manga::Manga,
        reviews::Review,
        users::UserUpdate
    },
    misc::*,
    response::Response,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaCharacter {
    pub character: Character,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaNews {
    pub data: Vec<NewsItem>,
    pub pagination: Pagination,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoreInfo {
    pub moreinfo: Option<String>,
}

impl JikanClient {
    pub async fn get_manga(&self, id: i32) -> Result<Response<Manga>, JikanError> {
        self.get(&format!("/manga/{}", id)).await
    }

    pub async fn get_manga_full(&self, id: i32) -> Result<Response<Manga>, JikanError> {
        self.get(&format!("/manga/{}/full", id)).await
    }

    pub async fn get_manga_characters(&self, id: i32) -> Result<Vec<MangaCharacter>, JikanError> {
        self.get(&format!("/manga/{}/characters", id)).await
    }

    pub async fn get_manga_news(
        &self,
        id: i32,
        page: Option<u32>,
    ) -> Result<MangaNews, JikanError> {
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
    ) -> Result<Response<Vec<Picture>>, JikanError> {
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
    ) -> Result<Response<Vec<Recommendation>>, JikanError> {
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
    ) -> Result<Response<Vec<Relation>>, JikanError> {
        self.get(&format!("/manga/{}/relations", id)).await
    }

    pub async fn get_manga_external(
        &self,
        id: i32,
    ) -> Result<Response<Vec<ExternalEntry>>, JikanError> {
        self.get(&format!("/manga/{}/external", id)).await
    }
}
