// Anime Type
pub enum AnimeType {
    TV,
    Movie,
    OVA,
    ONA,
    Music,
    CM,
    PV,
    TVSpecial,
}

impl AnimeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AnimeType::TV => "tv",
            AnimeType::Movie => "movie",
            AnimeType::OVA => "ova",
            AnimeType::ONA => "ona",
            AnimeType::Music => "music",
            AnimeType::CM => "cm",
            AnimeType::PV => "pv",
            AnimeType::TVSpecial => "tv_special",
        }
    }
}

// Anime Status
pub enum AnimeStatus {
    Airing,
    Complete,
    Upcoming
}

impl AnimeStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            AnimeStatus::Airing => "airing",
            AnimeStatus::Complete => "complete",
            AnimeStatus::Upcoming => "upcoming",
        }
    }
}

// Anime Rating
pub enum AnimeRating {
    G,
    PG,
    PG13,
    R17,
    R,
    Rx
}

impl AnimeRating {
    pub fn as_str(&self) -> &'static str {
        match self {
            AnimeRating::G => "g",
            AnimeRating::PG => "pg",
            AnimeRating::PG13 => "pg13",
            AnimeRating::R17 => "r17",
            AnimeRating::R => "r",
            AnimeRating::Rx => "rx",
        }
    }
}

// Anime Order By
// "mal_id" "title" "start_date" "end_date" "episodes" "score" "scored_by" "rank" "popularity" "members" "favorites"
pub enum AnimeOrder {
    MalId,
    Title,
    StartDate,
    EndDate,
    Episodes,
    Score,
    ScoredBy,
    Rank,
    Popularity,
    Members,
    Favourites
}

impl AnimeOrder {
    pub fn as_str(&self) -> &'static str {
        match self {
            AnimeOrder::MalId => "mal_id",
            AnimeOrder::Title => "title",
            AnimeOrder::StartDate => "start_date",
            AnimeOrder::EndDate => "end_date",
            AnimeOrder::Episodes => "episodes",
            AnimeOrder::Score => "score",
            AnimeOrder::ScoredBy => "scored_by",
            AnimeOrder::Rank => "rank",
            AnimeOrder::Popularity => "popularity",
            AnimeOrder::Members => "members",
            AnimeOrder::Favourites => "favourites",
        }
    }
}

// Anime Filter
// "airing" "upcoming" "bypopularity" "favorite"
pub enum AnimeFilter {
    Airing,
    Upcoming,
    ByPopularity,
    Favorite
}

impl AnimeFilter {
    pub fn as_str(&self) -> &'static str {
        match self {
            AnimeFilter::Airing => "airing",
            AnimeFilter::Upcoming => "upcoming",
            AnimeFilter::ByPopularity => "bypopularity",
            AnimeFilter::Favorite => "favorite",
        }
    }
}