use super::{
    class::Class, distance::Distance, gender_group::GenderGroup, pool_length::PoolLength,
    style::Style,
};
extern crate serde;
use self::serde::Deserialize;
use std::time::Duration;

/// Single qualification sets limits on who can enroll to a meet.
/// `Athlete`s `TimeResult` has to be less than `qualification_time` to
/// enroll.
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Qualification {
    /// Qualification is valid for this class.
    #[serde(rename = "Class")]
    pub athlete_class: Class,

    /// Qualification applies to this gender.
    #[serde(rename = "Sex")]
    pub athlete_gender: GenderGroup,

    /// Length of the pool of the `TimeResult` required.
    pub pool_length: PoolLength,

    /// Distance of the Event that the qualification applies to
    #[serde(rename = "DistanceLength")]
    pub distance: Distance,

    /// Style this qualification applies to.
    #[serde(rename = "Distanceart")]
    pub style: Style,

    /// Max time an athlete can have for successful enrollment.
    #[serde(rename = "QualificationTime", deserialize_with = "duration")]
    pub time: Duration,
}

/// Deserializer for Qualification structure. We need to use a custom deserializer because
/// `Duration` is not defined in this crate.
pub fn duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    const EXPECTED: &str = "6 character long string formatted like 'MMsshh' where 'MM' is minutes, 'ss' is seconds and 'hh' is hundredth part of a second. All values has leading zero.";
    let s: String = serde::de::Deserialize::deserialize(deserializer)?;

    // Assuming all numbers are inclusive 🤞
    // and fuck these error checks. Can this be somewhat shortened?
    let minutes = s[0..2].parse::<u64>().map_err(|err| -> D::Error {
        serde::de::Error::invalid_value(serde::de::Unexpected::Str(&err.to_string()), &EXPECTED)
    })?;
    let seconds = s[3..4].parse::<u64>().map_err(|err| -> D::Error {
        serde::de::Error::invalid_value(serde::de::Unexpected::Str(&err.to_string()), &EXPECTED)
    })?;
    let milliseconds = &s[5..6].parse::<u64>().map_err(|err| -> D::Error {
        serde::de::Error::invalid_value(serde::de::Unexpected::Str(&err.to_string()), &EXPECTED)
    })?;

    let duration = minutes * 60 * 1000 + seconds * 1000 + milliseconds * 100;
    Ok(Duration::from_millis(duration))
}
