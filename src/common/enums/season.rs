// Season Filter
// "tv" "movie" "ova" "special" "ona" "music"
pub enum SeasonFilter {
    Winter,
    Spring,
    Summer,
    Fall,
}

impl SeasonFilter {
    pub fn as_str(&self) -> &'static str {
        match self {
            SeasonFilter::Winter => "winter",
            SeasonFilter::Spring => "spring",
            SeasonFilter::Summer => "summer",
            SeasonFilter::Fall => "fall",
        }
    }
}