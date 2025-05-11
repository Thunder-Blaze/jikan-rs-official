//user.rs
use crate::{
    response::{Response, MalCommonResponse},
    enums::users::{Gender, UserHistoryType},
    structs::{
        anime::Anime, 
        character::Character, 
        manga::Manga, 
        people::Person, 
        recommendation::Recommendation, 
        reviews::Review
    }, utils::{ExternalEntry, Images}, JikanClient, JikanError
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserById {
    pub url: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAnimeInfo {
    pub days_watched: f32,
    pub mean_score: f32,
    pub watching: i32,
    pub completed: i32,
    pub on_hold: i32,
    pub dropped: i32,
    pub plan_to_watch: i32,
    pub total_entries: i32,
    pub rewatched: i32,
    pub episodes_watched: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMangaInfo {
    pub days_read: f32,
    pub mean_score: f32,
    pub reading: i32,
    pub completed: i32,
    pub on_hold: i32,
    pub dropped: i32,
    pub plan_to_read: i32,
    pub total_entries: i32,
    pub reread: i32,
    pub chapters_read: i32,
    pub volumes_read: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStats {
    pub anime: UserAnimeInfo,
    pub manga: UserMangaInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAbout {
    pub about: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserClub {
    pub mal_id: u32,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAnimeUpdates {
    pub entry: Anime,
    pub score: Option<i32>,
    pub status: String,
    pub episodes_seen: Option<u32>,
    pub episodes_total: Option<u32>,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMangaUpdates {
    pub entry: Manga,
    pub score: Option<i32>,
    pub status: String,
    pub chapters_read: Option<u32>,
    pub chapters_total: Option<u32>,
    pub volumes_read: Option<u32>,
    pub volumes_total: Option<u32>,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Friend {
    user: User,
    last_online: Option<String>,
    friends_since: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserHistory {
    pub entry: MalCommonResponse,
    pub increment: Option<i32>,
    pub date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFavorite {
    pub anime: Vec<Anime>,
    pub manga: Vec<Manga>,
    pub characters: Vec<Character>,
    pub people: Vec<Person>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdates {
    anime: Vec<UserAnimeUpdates>,
    manga: Vec<UserMangaUpdates>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub url: String,
    pub images: Option<Images>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewUser {
    pub url: String,
    pub username: String,
    pub images: Images,
}

pub struct GetUsersParams {
    pub page: Option<String>,
    pub limit: Option<u32>,
    pub q: Option<String>,
    pub gender: Option<Gender>,
    pub location: Option<String>,
    pub max_age: Option<u32>,
    pub min_age: Option<u32>,
}

impl JikanClient {
    pub async fn get_user_full(&self, username: &str) -> Result<Response<User>, JikanError> {
        self.get(&format!("/users/{}/full", username)).await
    }

    pub async fn get_user(&self, username: &str) -> Result<Response<User>, JikanError> {
        self.get(&format!("/users/{}", username)).await
    }

    pub async fn get_users(
        &self,
        param: GetUsersParams,
    ) -> Result<Response<Vec<User>>, JikanError> {
        let mut params = Vec::new();

        if let Some(g) = param.gender {
            params.push(format!("gender={}", g.as_str()));
        }

        if let Some(q_str) = param.q {
            params.push(format!("q={}", q_str));
        }

        if let Some(p) = param.page {
            params.push(format!("page={}", p));
        }

        if let Some(l) = param.limit {
            params.push(format!("limit={}", l));
        }

        if let Some(loc) = param.location {
            params.push(format!("location={}", loc));
        }

        if let Some(min_a) = param.min_age {
            params.push(format!("MinAge={}", min_a));
        }

        if let Some(max_a) = param.max_age {
            params.push(format!("MaxAge={}", max_a));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/users/{}", query)).await
    }

    pub async fn get_user_by_id(&self, id: i32) -> Result<Response<UserById>, JikanError> {
        //Maybe handle this better?
        self.get(&format!("/users/userbyid/{}", id)).await
    }

    pub async fn get_user_stats(&self, username: &str) -> Result<Response<UserStats>, JikanError> {
        self.get(&format!("/users/{}/statistics", username)).await
    }

    pub async fn get_user_updates(
        &self,
        username: &str,
    ) -> Result<Response<UserUpdates>, JikanError> {
        self.get(&format!("/users/{}/userupdates", username)).await
    }

    pub async fn get_user_friends(
        &self,
        username: &str,
        page: Option<u32>,
    ) -> Result<Response<Vec<Friend>>, JikanError> {
        let mut path = format!("/users/{}/friends", username);

        if let Some(p) = page {
            path = format!("/users/{}/friends?page={}", username, p);
        }

        self.get(&path).await
    }

    pub async fn get_user_reviews(
        &self,
        username: &str,
        page: Option<u32>,
    ) -> Result<Response<Vec<Review>>, JikanError> {
        let mut params = String::new();
        if let Some(p) = page {
            params = format!("?page={}", p);
        }
        self.get(&format!("/users/{}/reviews{}", username, params))
            .await
    }

    pub async fn get_user_recommendations(
        &self,
        username: &str,
        page: Option<u32>,
    ) -> Result<Response<Vec<Recommendation>>, JikanError> {
        let mut params = String::new();
        if let Some(p) = page {
            params = format!("?page={}", p);
        }
        self.get(&format!("/users/{}/recommendations{}", username, params))
            .await
    }

    pub async fn get_user_clubs(
        &self,
        username: &str,
        page: Option<u32>,
    ) -> Result<Response<Vec<UserClub>>, JikanError> {
        let mut params = String::new();
        if let Some(p) = page {
            params = format!("?page={}", p);
        }
        self.get(&format!("/users/{}/clubs{}", username, params))
            .await
    }

    pub async fn get_user_history(
        &self,
        username: &str,
        r#type: Option<UserHistoryType>,
    ) -> Result<Response<Vec<UserHistory>>, JikanError> {
        let mut params = Vec::new();

        if let Some(g) = r#type {
            params.push(format!("type={}", g.as_str()));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/users/{}/history{}", username, query)).await
    }

    pub async fn get_user_favorites(
        &self,
        username: &str,
    ) -> Result<Response<UserFavorite>, JikanError> {
        self.get(&format!("/users/{}/favorites", username)).await
    }

    pub async fn get_user_external(
        &self,
        username: &str,
    ) -> Result<Response<Vec<ExternalEntry>>, JikanError> {
        self.get(&format!("/users/{}/external", username)).await
    }

    pub async fn get_user_about(
        &self,
        username: &str,
    ) -> Result<Response<UserAbout>, JikanError> {
        self.get(&format!("/users/{}/about", username)).await
    }
}
