use serde::{Deserialize, Serialize};
use crate::utils::Images;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub url: String,
    pub images: Option<Images>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdate {
    pub user: User,
    pub score: Option<i32>,
    pub status: String,
    pub chapters_read: Option<i32>,
    pub chapters_total: Option<i32>,
    pub date: String,
}