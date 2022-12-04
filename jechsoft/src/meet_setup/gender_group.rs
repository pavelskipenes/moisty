use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum GenderGroup {
    /// restrict athletes to male participants only
    Male,
    /// restrict athletes to female participants only
    Female,
    /// both genders can enroll freely on individual events.
    /// Unconfirmed: Team relays
    Mixed,
}
#[allow(clippy::recursive_format_impl)]
impl fmt::Display for GenderGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match f.align() {
            None => match self {
                GenderGroup::Male => write!(f, "male"),
                GenderGroup::Female => write!(f, "female"),
                GenderGroup::Mixed => write!(f, "mixed"),
            },
            Some(_) => f.pad(&self.to_string()),
        }
    }
}
