// Schedule Filter
// "monday" "tuesday" "wednesday" "thursday" "friday" "saturday" "sunday" "unknown" "other"
pub enum ScheduleFilter {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
    Unknown,
    Other,
}

impl ScheduleFilter {
    pub fn as_str(&self) -> &'static str {
        match self {
            ScheduleFilter::Monday => "monday",
            ScheduleFilter::Tuesday => "tuesday",
            ScheduleFilter::Wednesday => "wednesday",
            ScheduleFilter::Thursday => "thursday",
            ScheduleFilter::Friday => "friday",
            ScheduleFilter::Saturday => "saturday",
            ScheduleFilter::Sunday => "sunday",
            ScheduleFilter::Unknown => "unknown",
            ScheduleFilter::Other => "other",
        }
    }
}