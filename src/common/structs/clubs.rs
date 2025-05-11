use crate::utils::Images;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Club {
    pub mal_id: i32,
    pub name: String,
    pub url: String,
    pub images: Option<Images>,
    pub members: Option<i32>,
    pub category: Option<String>,
    pub created: Option<String>,
    pub access: Option<String>,
}