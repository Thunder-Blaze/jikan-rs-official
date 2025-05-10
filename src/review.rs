use crate::{
    JikanClient, JikanError,
    response::Response,
    structs::reviews::Review,
};

impl JikanClient {
    pub async fn get_recent_anime_reviews(
        &self,
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

        self.get(&format!("/reviews/anime{}", query)).await
    }

    pub async fn get_recent_manga_reviews(
        &self,
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

        self.get(&format!("/reviews/manga{}", query)).await
    }
}
