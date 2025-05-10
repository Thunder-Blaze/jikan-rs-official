use serde::{Deserialize, Serialize};
use crate::utils::Images;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub relation: String,
    pub entry: Vec<RelatedEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedEntry {
    pub mal_id: i32,
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationResponse {
    pub data: Vec<Relation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub entry: RecommendationEntry,
    pub votes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationEntry {
    pub mal_id: i32,
    pub url: String,
    pub images: Images,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Score {
    pub score: i32,
    pub votes: i32,
    pub percentage: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Picture {
    pub jpg: Option<ImageFormat>,
    pub webp: Option<ImageFormat>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageFormat {
    pub image_url: String,
    pub small_image_url: String,
    pub large_image_url: String,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsItem {
    pub mal_id: i32,
    pub url: String,
    pub title: String,
    pub date: String,
    pub author_username: String,
    pub author_url: String,
    pub forum_url: String,
    pub images: Images,
    pub comments: i32,
    pub excerpt: String,
}
