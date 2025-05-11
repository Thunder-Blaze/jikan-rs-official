// Clubs Type
// "public" "private" "secret"
pub enum ClubType {
    Public,
    Private,
    Secret,
}

impl ClubType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ClubType::Public => "public",
            ClubType::Private => "private",
            ClubType::Secret => "secret",
        }
    }
}

// Club Category
// "anime" "manga" "actors_and_artists" "characters" "cities_and_neighborhoods" "companies" "conventions" "games" "japan" "music" "other" "schools"
pub enum ClubCategory {
    Anime,
    Manga,
    ActorsAndArtists,
    Characters,
    CitiesAndNeighborhoods,
    Companies,
    Conventions,
    Games,
    Japan,
    Music,
    Other,
    Schools,
}

impl ClubCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            ClubCategory::Anime => "anime",
            ClubCategory::Manga => "manga",
            ClubCategory::ActorsAndArtists => "actors_and_artists",
            ClubCategory::Characters => "characters",
            ClubCategory::CitiesAndNeighborhoods => "cities_and_neighborhoods",
            ClubCategory::Companies => "companies",
            ClubCategory::Conventions => "conventions",
            ClubCategory::Games => "games",
            ClubCategory::Japan => "japan",
            ClubCategory::Music => "music",
            ClubCategory::Other => "other",
            ClubCategory::Schools => "schools",
        }
    }
}

// Club Order By
// "mal_id" "name" "members_count" "created"
pub enum ClubOrder {
    MalId,
    Name,
    MembersCount,
    Created,
}

impl ClubOrder {
    pub fn as_str(&self) -> &'static str {
        match self {
            ClubOrder::MalId => "mal_id",
            ClubOrder::Name => "name",
            ClubOrder::MembersCount => "members_count",
            ClubOrder::Created => "created",
        }
    }
}
