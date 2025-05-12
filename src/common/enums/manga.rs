// Manga Type
// "manga" "novel" "lightnovel" "oneshot" "doujin" "manhwa" "manhua"
pub enum MangaType {
    Manga,
    Novel,
    LightNovel,
    OneShot,
    Doujin,
    Manhwa,
    Manhua,
}

impl MangaType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MangaType::Manga => "manga",
            MangaType::Novel => "novel",
            MangaType::LightNovel => "lightnovel",
            MangaType::OneShot => "oneshot",
            MangaType::Doujin => "doujin",
            MangaType::Manhwa => "manhwa",
            MangaType::Manhua => "manhua",
        }
    }
}

// Manga Status
// "publishing" "complete" "hiatus" "discontinued" "upcoming"
pub enum MangaStatus {
    Publishing,
    Complete,
    Hiatus,
    Discontinued,
    Upcoming,
}
impl MangaStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            MangaStatus::Publishing => "publishing",
            MangaStatus::Complete => "complete",
            MangaStatus::Hiatus => "hiatus",
            MangaStatus::Discontinued => "discontinued",
            MangaStatus::Upcoming => "upcoming",
        }
    }
}

// Manga Order By
// "mal_id" "title" "start_date" "end_date" "chapters" "volumes" "score" "scored_by" "rank" "popularity" "members" "favorites"
pub enum MangaOrder {
    MalId,
    Title,
    StartDate,
    EndDate,
    Chapters,
    Volumes,
    Score,
    ScoredBy,
    Rank,
    Popularity,
    Members,
    Favorites,
}
impl MangaOrder {
    pub fn as_str(&self) -> &'static str {
        match self {
            MangaOrder::MalId => "mal_id",
            MangaOrder::Title => "title",
            MangaOrder::StartDate => "start_date",
            MangaOrder::EndDate => "end_date",
            MangaOrder::Chapters => "chapters",
            MangaOrder::Volumes => "volumes",
            MangaOrder::Score => "score",
            MangaOrder::ScoredBy => "scored_by",
            MangaOrder::Rank => "rank",
            MangaOrder::Popularity => "popularity",
            MangaOrder::Members => "members",
            MangaOrder::Favorites => "favorites",
        }
    }
}

// Manga Filter
// "publishing" "upcoming" "bypopularity" "favorite"
pub enum MangaFilter {
    Publishing,
    Upcoming,
    ByPopularity,
    Favorite,
}

impl MangaFilter {
    pub fn as_str(&self) -> &'static str {
        match self {
            MangaFilter::Publishing => "publishing",
            MangaFilter::Upcoming => "upcoming",
            MangaFilter::ByPopularity => "bypopularity",
            MangaFilter::Favorite => "favorite",
        }
    }
}
