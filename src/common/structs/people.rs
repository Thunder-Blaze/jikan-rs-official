use serde::{Deserialize, Serialize};
use crate::{
    utils::Images,
    structs::{
        anime::Anime,
        manga::Manga,
        character::Character,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonAnimePosition {
    pub position: String,
    pub anime: Anime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonMangaPosition {
    pub position: String,
    pub manga: Manga,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonVoiceActingRole {
    pub role: String,
    pub anime: Anime,
    pub character: Character,
}
    
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub mal_id: i32,
    pub url: String,
    pub website_url: Option<String>,
    pub images: Images,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub alternate_names: Option<Vec<String>>,
    pub birthday: Option<String>,
    pub favories: Option<i32>,
    pub about: Option<String>,
    pub anime: Option<Vec<PersonAnimePosition>>,
    pub manga: Option<Vec<PersonMangaPosition>>,
    pub voice_acting_roles: Option<Vec<PersonVoiceActingRole>>,
}