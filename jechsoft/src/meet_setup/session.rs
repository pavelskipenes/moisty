extern crate chrono;
extern crate serde;
extern crate time;
use self::chrono::NaiveDate;
use self::serde::Deserialize;
use self::time::Time;
use super::deserializer;

/// `Session` is a set of continuous `Event`s without a break.
/// A `Meet` have at least one `Session` and at most one `Session` per `Event`.
#[derive(Deserialize, Debug, Clone)]
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
