// Season Filter
// "tv" "movie" "ova" "special" "ona" "music"
pub enum SeasonFilter {
    TV,
    Movie,
    OVA,
    Special,
    ONA,
    Music,
}

impl SeasonFilter {
    pub fn as_str(&self) -> &str {
        match self {
            SeasonFilter::TV => "tv",
            SeasonFilter::Movie => "movie",
            SeasonFilter::OVA => "ova",
            SeasonFilter::Special => "special",
            SeasonFilter::ONA => "ona",
            SeasonFilter::Music => "music",
        }
    }
}
