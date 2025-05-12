use crate::{
    common::utils::ExternalEntry, response::MalCommonResponse, structs::{anime::Anime, character::Character, manga::Manga, people::Person}, utils::Images
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub url: String,
    pub images: Option<Images>,
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
pub struct UserUpdate {
    pub user: User,
    pub score: Option<u32>,
    pub status: String,
    pub chapters_read: Option<u32>,
    pub chapters_total: Option<u32>,
    pub episodes_seen: Option<u32>,
    pub episodes_total: Option<u32>,
    pub date: String,
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
pub struct UserAbout {
    pub about: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserExtended {
    mal_id: u32,
    username: String,
    url: String,
    images: Option<Images>,
    last_online: String,
    gender: Option<String>,
    birthday: Option<String>,
    location: Option<String>,
    joined: String,
    statistics: Option<UserStats>,
    external: Option<Vec<ExternalEntry>>,
}