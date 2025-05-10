// Gender
// "any" "male" "female" "nonbinary"
pub enum Gender {
    Any,
    Male,
    Female,
    Nonbinary,
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