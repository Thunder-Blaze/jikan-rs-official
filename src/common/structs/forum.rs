use serde::{Deserialize, Serialize};
use crate::utils::Images;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumTopic {
    pub mal_id: i32,
    pub url: String,
    pub title: String,
    pub date: String,
    pub author_username: String,
    pub author_url: String,
    pub comments: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsItem {
    pub mal_id: i32,
    pub url: String,
    pub title: String,
    pub date: String,
    pub author_username: String,
    pub author_url: String,
    pub comments: i32,
    pub forum_url: String,
    pub images: Images,
    pub excerpt: String,
}