use serde::Deserialize;

use super::qualification::Qualification;

#[derive(Debug, Deserialize)]
pub struct QualificationSet {
    #[serde(rename = "SetName")]
    pub name: String,
    #[serde(rename = "Qualification")]
    pub qualifications: Vec<Qualification>,
}
