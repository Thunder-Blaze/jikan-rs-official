use serde::{Deserialize, Serialize};
use crate::utils::{Images, Title, ExternalEntry};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Producer {
    pub mal_id: i32,
    pub url: String,
    pub titles: Vec<Title>,
    pub images: Images,
    pub favorites: i32,
    pub count: i32,
    pub established: Option<String>,
    pub about: Option<String>,
    pub external: Option<Vec<ExternalEntry>>,
}