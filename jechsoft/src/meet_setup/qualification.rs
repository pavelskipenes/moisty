use std::time::Duration;

use serde::Deserialize;

use super::{
    class::Class, distance::Distance, gender_group::GenderGroup, pool_length::PoolLength,
    style::Style,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Qualification {
    /// Undocumented field.
    pub class: Class,

    /// Undocumented field.
    #[serde(deserialize_with = "duration")]
    pub qualification_time: Duration,

    /// Undocumented field.
    pub pool_length: PoolLength,

    /// Undocumented field.
    #[serde(rename = "Sex")]
    pub gender: GenderGroup,

    /// Undocumented field.
    #[serde(rename = "DistanceLength")]
    pub distance: Distance,

    /// Undocumented field.
    #[serde(rename = "Distanceart")]
    pub style: Style,
}

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
