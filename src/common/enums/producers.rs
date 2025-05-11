// Producers Order By
// "mal_id" "count" "favorites" "established"
pub enum ProducersOrder {
    MalId,
    Count,
    Favorites,
    Established,
}

impl ProducersOrder {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProducersOrder::MalId => "mal_id",
            ProducersOrder::Count => "count",
            ProducersOrder::Favorites => "favorites",
            ProducersOrder::Established => "established",
        }
    }
}
