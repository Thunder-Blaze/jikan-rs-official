// Magazine Order By
pub enum MagazineOrder {
    MalId,
    Name,
    Count,
}

impl MagazineOrder {
    pub fn as_str(&self) -> &'static str {
        match self {
            MagazineOrder::MalId => "mal_id",
            MagazineOrder::Name => "name",
            MagazineOrder::Count => "count",
        }
    }
}