// Character Order By
// "mal_id" "name" "favorites"
pub enum CharacterOrder {
    MalId,
    Name,
    Favorites,
}

impl CharacterOrder {
    pub fn as_str(&self) -> &'static str {
        match self {
            CharacterOrder::MalId => "mal_id",
            CharacterOrder::Name => "name",
            CharacterOrder::Favorites => "favorites",
        }
    }
}
