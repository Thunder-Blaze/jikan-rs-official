use crate::{
    JikanClient, JikanError, enums::genre::GenreFilter, response::Response, structs::genre::Genre,
};

impl JikanClient {
    pub async fn get_anime_genres(
        &self,
        filter: Option<GenreFilter>,
    ) -> Result<Response<Vec<Genre>>, JikanError> {
        let mut query = String::new();

        if let Some(q) = filter {
            query = format!("?filter={}", q.as_str());
        }

        self.get(&format!("/genres/anime{}", query)).await
    }

    pub async fn get_manga_genres(
        &self,
        filter: Option<GenreFilter>,
    ) -> Result<Response<Vec<Genre>>, JikanError> {
        let mut query = String::new();

        if let Some(q) = filter {
            query = format!("?filter={}", q.as_str());
        }

        self.get(&format!("/genres/manga{}", query)).await
    }
}
