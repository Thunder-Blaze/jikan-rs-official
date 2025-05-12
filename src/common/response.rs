use crate::utils::Pagination;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    pub data: T,
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MalCommonResponse {
    pub mal_id: u32,
    pub r#type: Option<String>,
    pub name: String,
    pub url: String,
}
