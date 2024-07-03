extern crate chrono;
extern crate gregorian;
extern crate serde;
extern crate time;
use self::chrono::NaiveDate;
use self::gregorian::Year;
use self::serde::Deserialize;
use self::time::{format_description::FormatItem, macros::format_description, Time};
use super::{event::Event, session::Session};
use std::time::Duration;

/// # Returns
/// returns `Ok(true)` if the input is "TRUE" and `Ok(false)` if the input is "FALSE"
///
/// # Errors
/// Returns an error if string is not "TRUE" or "FALSE"
pub fn bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: String = serde::de::Deserialize::deserialize(deserializer)?;

    match s.as_str() {
        "TRUE" => Ok(true),
        "FALSE" => Ok(false),
        _ => Err(serde::de::Error::unknown_variant(&s, &["TRUE", "FALSE"])),
    }
}

/// # Returns
/// returns `Some(true)` if the input is "TRUE"
/// returns `Some(false)` if the input is "FALSE"
/// returns `None` if no input were given
///
/// # Errors
/// Returns an error if input is neither "TRUE" or "FALSE"
pub fn option_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: Option<String> = serde::de::Deserialize::deserialize(deserializer)?;
    s.map_or_else(
        || Ok(None),
        |s| match s.as_str() {
            "TRUE" => Ok(Some(true)),
            "FALSE" => Ok(Some(false)),
            _ => Err(serde::de::Error::unknown_variant(
                &s,
                &["TRUE", "FALSE", ""],
            )),
        },
    )
}

/// # Returns
/// Will return `None` if the parsed string is empty.
/// Will return a `Ok(NaiveDate)` if the date is parsed successfully.
///
/// # Errors
/// This deserializer will return an error if a date does not follow the epected jechsoft format.
/// Expected input is a string of 8 characters in total. Allowed characters are only numbers
/// 0-9. The first four digits needs to represent a full Gregorian year with leading
/// zeroes. The next two digits can be in range 0 - 12 with leading zero representing the month of
/// the year. The last two digits needs to be day number in the month. Allowed range is between 1
/// and 31 inclusive and with leading zeroes.
pub fn option_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    match s.as_str() {
        "" => Ok(None),
        date_maybe => match NaiveDate::parse_from_str(date_maybe, "%Y%m%d") {
            Ok(date) => Ok(Some(date)),
            Err(err) => Err(serde::de::Error::custom(err.to_string())),
        },
    }
}

/// # Errors
/// This deserializer will return an error if a date does not follow the epected jechsoft format.
/// Expected input is a string of 8 characters in total. Allowed characters are only numbers
/// 0-9. The first four digits needs to represent a full Gregorian year with leading
/// zeroes. The next two digits can be in range 0 - 12 with leading zero representing the month of
/// the year. The last two digits needs to be day number in the month. Allowed range is between 1
/// and 31 inclusive and with leading zeroes.
pub fn date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    match NaiveDate::parse_from_str(&s, "%Y%m%d") {
        Ok(date) => Ok(date),
        Err(err) => Err(serde::de::Error::custom(err.to_string())),
    }
}

/// # Errors
/// Will return an error if the desiaralized string cannot be parsed as a date.
pub fn option_year<'de, D>(deserializer: D) -> Result<Option<Year>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    match s.parse::<i16>() {
        Ok(year) => Ok(Some(Year::new(year))),
        Err(err) => Err(serde::de::Error::custom(err.to_string())),
    }
}

/// # Time deserielizer
/// parses text and returns Time of the parsed value is a four character long string containing
/// only numbers, the first two digits represent the hour of the day and the second pair represents
/// the minute of the hour. Leading zeroes are required for correct parsing.
///
/// # Errors
/// returns an error if parsing an empty string, and if time cannot be pared as expected
pub fn time<'de, D>(deserializer: D) -> Result<Time, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    const EXPECTED: &str = "a 4 char long string containing numbers representing time of day formatted as 24 hour ('hhmm') with leading zeroes. Min value '0000' max value '2359'.";
    const FORMAT: &[FormatItem] = format_description!("[hour][minute]");

    let deserialized_string: String = Deserialize::deserialize(deserializer)?;

    if deserialized_string.is_empty() {
        return Err(serde::de::Error::custom(format!(
            "received an empty string, expected {EXPECTED}"
        )));
    }
    let result = Time::parse(&deserialized_string, &FORMAT).map_err(|err| -> D::Error {
        serde::de::Error::invalid_value(serde::de::Unexpected::Str(&err.to_string()), &EXPECTED)
    })?;

    Ok(result)
}

/// # Errors
/// returns an error if parsing string to number fails.
pub fn option_duration<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    const EXPECTED: &str = "8 character long string formatted like 'MM:ss:hh' where 'MM' is minutes, 'ss' is seconds and 'hh' is hundredth part of a second. All values has to be present and separated by ':' with leading zero.";
    let s: String = serde::de::Deserialize::deserialize(deserializer)?;

    // Assuming all numbers are inclusive 🤞
    // and fuck these error checks. Can this be somewhat shortened?
    let minutes = s[0..2].parse::<u64>().map_err(|err| -> D::Error {
        serde::de::Error::invalid_value(serde::de::Unexpected::Str(&err.to_string()), &EXPECTED)
    })?;
    let seconds = s[4..5].parse::<u64>().map_err(|err| -> D::Error {
        serde::de::Error::invalid_value(serde::de::Unexpected::Str(&err.to_string()), &EXPECTED)
    })?;
    let milliseconds = &s[7..8].parse::<u64>().map_err(|err| -> D::Error {
        serde::de::Error::invalid_value(serde::de::Unexpected::Str(&err.to_string()), &EXPECTED)
    })?;

    let duration = minutes * 60 * 1000 + seconds * 1000 + milliseconds * 100;
    Ok(Some(Duration::from_millis(duration)))
}

/// # Errors
/// returns an error if deserialization fails.
pub fn session<'de, D>(deserializer: D) -> Result<Vec<Session>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper {
        #[serde(rename = "Session", default)]
        sessions: Vec<Session>,
    }

    let wrapper: Wrapper = Deserialize::deserialize(deserializer)?;

    Ok(wrapper.sessions)
}

/// # Errors
/// returns an error if deserialization fails
pub fn event<'de, D>(deserializer: D) -> Result<Vec<Event>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct UselessWrapper {
        #[serde(rename = "Event")]
        events: Vec<Event>,
    }

    let wrapper: UselessWrapper = Deserialize::deserialize(deserializer)?;

    Ok(wrapper.events)
}

/// # Errors
/// returns an error if the input cannot be parsed into a `Year`
/// # Bugs
/// possible bug when `OnePriceAllClasses` is empty, which might return Ok(Some([])) instead of Ok(None)
pub fn one_price_all_class<'de, D>(deserializer: D) -> Result<Option<Vec<Year>>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct UselessWrapper {
        #[serde(rename = "OnePriceAllClasses", default)]
        birth_years: Vec<String>,
    }

    let wrapper: UselessWrapper = Deserialize::deserialize(deserializer)?;
    let mut new_vec = vec![];
    for numeric_string in wrapper.birth_years {
        let year = match numeric_string.parse::<i16>() {
            Ok(year) => Year::new(year),
            Err(why) => Err(serde::de::Error::custom(why.to_string()))?,
        };

        new_vec.push(year);
    }
    Ok(Some(new_vec))
}
