// Sort
pub enum Sort {
    Asc,
    Desc,
}

impl Sort {
    pub fn as_str(&self) -> &'static str {
        match self {
            Sort::Asc => "asc",
            Sort::Desc => "desc",
        }
    }
}