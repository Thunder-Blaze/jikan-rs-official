// Genre Filter
// genres" "explicit_genres" "themes" "demographics"
pub enum GenreFilter {
    Genres,
    ExplicitGenres,
    Themes,
    Demographics,
}

impl GenreFilter {
    pub fn as_str(&self) -> &str {
        match self {
            GenreFilter::Genres => "genres",
            GenreFilter::ExplicitGenres => "explicit_genres",
            GenreFilter::Themes => "themes",
            GenreFilter::Demographics => "demographics",
        }
    }
}
