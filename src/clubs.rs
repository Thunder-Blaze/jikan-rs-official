use crate::{
    JikanClient, JikanError,
    utils::{Images, Pagination},
    response::Response,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubVectorResponse<T> {
    pub data: Vec<T>,
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Club {
    pub mal_id: i32,
    pub url: String,
    pub images: Images,
    pub name: String,
    pub members: i32,
    pub category: String,
    pub created: String,
    pub access: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubMember {
    pub username: String,
    pub url: String,
    pub images: Images,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubStaff {
    pub url: String,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubRelationContent {
    pub mal_id: i32,
    pub r#type: Option<String>,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubRelations {
    pub anime: Vec<ClubRelationContent>,
    pub manga: Vec<ClubRelationContent>,
    pub characters: Vec<ClubRelationContent>,
}

// New struct for club search parameters
#[derive(Debug, Clone, Default)]
pub struct ClubSearchParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub q: Option<String>,
    pub category: Option<String>,
    pub order_by: Option<String>,
    pub sort: Option<String>,
    pub letter: Option<String>,
}

impl ClubSearchParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_query(mut self, q: impl Into<String>) -> Self {
        self.q = Some(q.into());
        self
    }

    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    pub fn with_order_by(mut self, order_by: impl Into<String>) -> Self {
        self.order_by = Some(order_by.into());
        self
    }

    pub fn with_sort(mut self, sort: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self
    }

    pub fn with_letter(mut self, letter: impl Into<String>) -> Self {
        self.letter = Some(letter.into());
        self
    }

    fn to_query_params(&self) -> String {
        let mut params = Vec::new();

        if let Some(p) = self.page {
            params.push(format!("page={}", p));
        }

        if let Some(l) = self.limit {
            params.push(format!("limit={}", l));
        }

        if let Some(qr) = &self.q {
            params.push(format!("q={}", qr));
        }

        if let Some(c) = &self.category {
            params.push(format!("category={}", c));
        }

        if let Some(ob) = &self.order_by {
            params.push(format!("order_by={}", ob));
        }

        if let Some(s) = &self.sort {
            params.push(format!("sort={}", s));
        }

        if let Some(lt) = &self.letter {
            params.push(format!("letter={}", lt));
        }

        if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        }
    }
}

impl JikanClient {
    pub async fn get_club_by_id(&self, id: i32) -> Result<Response<Club>, JikanError> {
        self.get(&format!("/clubs/{}", id)).await
    }

    pub async fn get_club_members(
        &self,
        id: i32,
        page: Option<u32>,
    ) -> Result<ClubVectorResponse<ClubMember>, JikanError> {
        let mut params = Vec::new();
        if let Some(p) = page {
            params.push(format!("page={}", p));
        }
        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };
        self.get(&format!("/clubs/{}/members{}", id, query)).await
    }

    pub async fn get_club_staff(
        &self,
        id: i32,
    ) -> Result<ClubVectorResponse<ClubStaff>, JikanError> {
        self.get(&format!("/clubs/{}/staff", id)).await
    }

    pub async fn get_club_relations(
        &self,
        id: i32,
    ) -> Result<Response<ClubRelations>, JikanError> {
        self.get(&format!("/clubs/{}/relations", id)).await
    }

    // Refactored search function that takes a struct instead of many parameters
    pub async fn get_club_search(
        &self,
        params: ClubSearchParams,
    ) -> Result<ClubVectorResponse<Club>, JikanError> {
        let query = params.to_query_params();
        self.get(&format!("/clubs{}", query)).await
    }
}
