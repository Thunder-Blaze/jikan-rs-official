// People Order By
// "mal_id" "name" "birthday" "favorites"
pub enum PeopleOrder {
    MalId,
    Name,
    Birthday,
    Favorites,
}

impl PeopleOrder {
    pub fn as_str(&self) -> &'static str {
        match self {
            PeopleOrder::MalId => "mal_id",
            PeopleOrder::Name => "name",
            PeopleOrder::Birthday => "birthday",
            PeopleOrder::Favorites => "favorites",
        }
    }
}
