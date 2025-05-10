use crate::{
    JikanClient,
    JikanError,
    response::Response,
    structs::recommendation::Recommendation,
};

impl JikanClient {
    pub async fn get_recent_anime_recommendations(
        &self,
        page: Option<u32>,
    ) -> Result<Response<Vec<Recommendation>>, JikanError> {
        let mut params = Vec::new();

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/recommendations/anime{}", query)).await
    }

    pub async fn get_recent_manga_recommendations(
        &self,
        page: Option<u32>,
    ) -> Result<Response<Vec<Recommendation>>, JikanError> {
        let mut params = Vec::new();

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/recommendations/manga{}", query)).await
    }
}
