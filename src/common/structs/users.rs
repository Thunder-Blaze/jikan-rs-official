use serde::{Deserialize, Serialize};
use crate::utils::Images;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub url: String,
    pub images: Option<Images>,
}