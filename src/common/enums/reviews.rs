// Reviews Type
// "anime" "manga"
pub enum ReviewType {
    Anime,
    Manga,
}

impl ReviewType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReviewType::Anime => "anime",
            ReviewType::Manga => "manga",
        }
    }
}
