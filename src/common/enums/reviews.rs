// Reviews Type
// "anime" "manga"
pub enum ReviewsType {
    Anime,
    Manga,
}

impl ReviewsType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReviewsType::Anime => "anime",
            ReviewsType::Manga => "manga",
        }
    }
}