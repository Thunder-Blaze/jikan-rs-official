use crate::utils::Images;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastComment {
    pub url: String,
    pub date: String,
    pub author_username: String,
    pub author_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumTopic {
    pub mal_id: i32,
    pub url: String,
    pub title: String,
    pub date: String,
    pub author_username: String,
    pub author_url: String,
    pub comments: i32,
    pub last_comment: Option<LastComment>,
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
