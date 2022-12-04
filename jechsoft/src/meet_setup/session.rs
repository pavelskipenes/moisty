use super::deserializer;
use chrono::NaiveDate;
use serde::Deserialize;
use time::Time;

#[derive(Deserialize, Debug)]
pub struct Session {
    /// Session id.
    #[serde(rename = "SessionId")]
    pub id: u8,

    /// Session name / title.
    #[serde(rename = "SessionName", default)]
    pub name: String,

    /// Session start date.
    #[serde(rename = "SessionDate", deserialize_with = "deserializer::date")]
    pub date: NaiveDate,

    /// Session start time.
    #[serde(rename = "SessionStartTime", deserialize_with = "deserializer::time")]
    pub start_time: Time,
}
