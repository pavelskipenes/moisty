use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AustralianWorldRecord {
    LongCourse,
    Same,
}

#[allow(clippy::recursive_format_impl)]
impl Display for AustralianWorldRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match f.align() {
            Some(_) => f.pad(&self.to_string()),
            None => write!(
                f,
                "{}",
                match self {
                    AustralianWorldRecord::LongCourse => "[undocumented] long-course",
                    AustralianWorldRecord::Same => "[undocumented] same",
                }
            ),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AustralianRank {
    Percent,
}

#[allow(clippy::recursive_format_impl)]
impl Display for AustralianRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match f.align() {
            Some(_) => f.pad(&self.to_string()),
            None => write!(
                f,
                "{}",
                match self {
                    AustralianRank::Percent => "[undocumented] percent",
                }
            ),
        }
    }
}
