// Forum Filter
pub enum ForumFilter {
    All,
    Episode,
    Other,
}

impl ForumFilter {
    pub fn as_str(&self) -> &'static str {
        match self {
            ForumFilter::All => "all",
            ForumFilter::Episode => "episode",
            ForumFilter::Other => "other",
        }
    }
}
