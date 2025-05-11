// magazines.rs
use crate::{
    JikanClient, JikanError, enums::common::Sort, enums::magazines::MagazineOrder,
    response::Response, structs::magazines::Magazine,
};

impl JikanClient {
    pub async fn get_magazines(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
        q: Option<String>,
        order_by: Option<MagazineOrder>,
        sort: Option<Sort>,
        letter: Option<String>,
    ) -> Result<Response<Vec<Magazine>>, JikanError> {
        let mut params = Vec::new();

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }

        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }

        if let Some(qr) = q {
            params.push(format!("q={}", qr));
        }

        if let Some(ob) = order_by {
            params.push(format!("order_by={}", ob.as_str()));
        }

        if let Some(s) = sort {
            params.push(format!("sort={}", s.as_str()));
        }

        if let Some(lt) = letter {
            params.push(format!("letter={}", lt));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/magazines{}", query)).await
    }
}
