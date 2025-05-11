// Gender
// "any" "male" "female" "nonbinary"
pub enum Gender {
    Any,
    Male,
    Female,
    NonBinary,
}

impl Gender {
    pub fn as_str(&self) -> &'static str {
        match self {
            Gender::Any => "any",
            Gender::Male => "male",
            Gender::Female => "female",
            Gender::Nonbinary => "nonbinary",
        }
    }
}

// User History Type
// "anime" "manga"
pub enum UserHistoryType {
    Anime,
    Manga,
}

impl UserHistoryType {
    pub fn as_str(&self) -> &'static str {
        match self {
            UserHistoryType::Anime => "anime",
            UserHistoryType::Manga => "manga",
        }
    }
}