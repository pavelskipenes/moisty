use super::qualification::Qualification;
extern crate serde;
use self::serde::Deserialize;

/// Qualification set contains a list of qualification for enrollment.
/// `Athlete`s that does not have a valid `TimeResult` for the meet cannot
/// enroll to the meet.
#[derive(Debug, Deserialize, Clone)]
pub struct QualificationSet {
    /// name of the qualification set
    #[serde(rename = "SetName")]
    pub name: String,

    /// List of qualifications
    #[serde(rename = "Qualification")]
    pub qualifications: Vec<Qualification>,
}
