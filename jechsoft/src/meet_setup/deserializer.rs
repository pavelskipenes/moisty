use super::{event::Event, session::Session, touch_pad_set::TouchPadSet};
use chrono::NaiveDate;
use datetime::Year;
use serde::Deserialize;
use std::time::Duration;
use time::{format_description::FormatItem, macros::format_description, Time};

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

pub fn option_year<'de, D>(deserializer: D) -> Result<Option<Year>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    match s.parse::<i64>() {
        Ok(year) => Ok(Some(datetime::Year(year))),
        Err(err) => Err(serde::de::Error::custom(err.to_string())),
    }
}

pub fn time<'de, D>(deserializer: D) -> Result<Time, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    const EXPECTED: &str = "a 4 char long string containing numbers representing time of day formatted as 24 hour ('hhmm') with leading zeroes. Min value '0000' max value '2359'.";
    const FORMAT: &[FormatItem] = format_description!("[hour][minute]");

    let deserialized_string: String = Deserialize::deserialize(deserializer)?;
    let result = Time::parse(&deserialized_string, &FORMAT).map_err(|err| -> D::Error {
        serde::de::Error::invalid_value(serde::de::Unexpected::Str(&err.to_string()), &EXPECTED)
    })?;

    Ok(result)
}

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

pub fn touch_pad_set<'de, D>(deserializer: D) -> Result<Option<TouchPadSet>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: String = serde::de::Deserialize::deserialize(deserializer)?;
    match s.as_str() {
        "ONE SET" => Ok(Some(TouchPadSet::OneSet)),
        "NO" => Ok(None),
        string => Err(serde::de::Error::custom(format!(
            "Could not decode {string} as TouchPadSet type"
        ))),
    }
}

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

pub fn one_price_all_class<'de, D>(deserializer: D) -> Result<Option<Vec<Year>>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct UselessWrapper {
        #[serde(rename = "OnePriceAllClasses", default)]
        core: Vec<String>,
    }

    let wrapper: UselessWrapper = Deserialize::deserialize(deserializer)?;

    let mut new_vec = vec![];
    for numeric_string in wrapper.core {
        let year = match numeric_string.parse::<i64>() {
            Ok(year) => datetime::Year(year),
            Err(why) => Err(serde::de::Error::custom(why.to_string()))?,
        };

        new_vec.push(year);
    }
    Ok(Some(new_vec))
}
