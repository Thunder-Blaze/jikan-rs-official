use serde::{Deserialize, Serialize};
use crate::{
    utils::Images,
    structs::anime::Anime,
    structs::manga::Manga,
    structs::people::Person,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub name: String,
    pub name_kanji: Option<String>,
    pub nicknames: Option<Vec<String>>,
    pub favorites: Option<u32>,
    pub about: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimeEntry {
    pub role: String,
    pub anime: Anime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaEntry {
    pub role: String,
    pub manga: Manga,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonEntry {
    pub language: String,
    pub person: Person,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterExtended {
    pub mal_id: u32,
    pub url: String,
    pub images: Images,
    pub name: String,
    pub name_kanji: Option<String>,
    pub nicknames: Option<Vec<String>>,
    pub favorites: Option<u32>,
    pub about: Option<String>,
    pub anime: Option<Vec<AnimeEntry>>,
    pub manga: Option<Vec<MangaEntry>>,
    pub voices: Option<Vec<PersonEntry>>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterRole {
    pub character: Character,
    pub role: String,
}