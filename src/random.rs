//random.rs
use crate::{
    JikanClient, JikanError,
    response::Response,
    structs::{anime::Anime, character::Character, manga::Manga, people::Person, users::User},
};

impl JikanClient {
    pub async fn get_random_anime(&self) -> Result<Response<Anime>, JikanError> {
        self.get("/random/anime").await
    }

    pub async fn get_random_manga(&self) -> Result<Response<Manga>, JikanError> {
        self.get("/random/manga").await
    }

    pub async fn get_random_user(&self) -> Result<Response<User>, JikanError> {
        self.get("/random/users").await
    }

    pub async fn get_random_character(&self) -> Result<Response<Character>, JikanError> {
        self.get("/random/characters").await
    }

    pub async fn get_random_person(&self) -> Result<Response<Person>, JikanError> {
        self.get("/random/people").await
    }

    pub async fn get_random_people(&self) -> Result<Response<Person>, JikanError> {
        // same as api name
        self.get("/random/people").await
    }
}
