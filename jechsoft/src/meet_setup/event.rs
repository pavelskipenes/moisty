extern crate chrono;
extern crate gregorian;
extern crate serde;
use self::chrono::NaiveDate;
use self::gregorian::Year;
use self::serde::Deserialize;
use super::{
    award::Award, deserializer, distance::Distance, gender_group::GenderGroup,
    pool_length::PoolLength, round::Round, sorting::Sorting, style::Style,
};
use std::time::Duration;

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
#[allow(clippy::struct_excessive_bools)]
#[serde(rename_all = "PascalCase")]

/// Stored configuration for Event inside `meetsetup.xml`
pub struct Event {
    /// Event number. Starts with 1.
    #[serde(rename = "EventNumber")]
    pub id: u32,

    /// Human readable description of the event
    #[serde(rename = "EventDescription")]
    pub description: String,

    /// Distance
    #[serde(rename = "EventLength")]
    pub distance: Distance,

    /// Swimming style
    #[serde(rename = "Eventart", alias = "EventArt")]
    pub style: Style,

    /// Defines which groups can compete in this event.
    #[serde(rename = "Sex")]
    pub gender_group: GenderGroup,

    #[serde(deserialize_with = "deserializer::bool")]
    pub senior: bool,

    #[serde(deserialize_with = "deserializer::bool")]
    pub junior: bool,

    #[serde(deserialize_with = "deserializer::bool")]
    pub junior_older: bool,

    #[serde(deserialize_with = "deserializer::bool")]
    pub junior_younger: bool,

    #[serde(deserialize_with = "deserializer::option_year", default)]
    pub youngest: Option<Year>,

    #[serde(deserialize_with = "deserializer::option_year", default)]
    pub oldest: Option<Year>,

    #[serde(rename = "EventPoolLength")]
    pub pool_length: PoolLength,

    /// Starting date for the meet
    #[serde(deserialize_with = "deserializer::date")]
    pub date: NaiveDate,

    // TODO: members `qualification_time_long_course` and `qualification_time_short_course`
    // should probably be merged together as a QualificationTime struct where
    // method would provide a duration for either short course or long course
    /// Optional qualification time for this event.
    #[serde(
        default,
        rename = "QualLongCourse",
        deserialize_with = "deserializer::option_duration"
    )]
    pub qualification_time_long_course: Option<Duration>,

    #[serde(
        default,
        rename = "QualShortCourse",
        deserialize_with = "deserializer::option_duration"
    )]
    pub qualification_time_short_course: Option<Duration>,

    pub sorting: Sorting,

    /// No qualification for handicap.
    #[serde(deserialize_with = "deserializer::bool", rename = "NoQualHcEvent")]
    pub no_qualification_for_handicap: bool,

    #[serde(deserialize_with = "deserializer::bool", rename = "Webheat")]
    pub web_heat: bool,

    /// Event sponsor text. Hint for other applications that might use this information if it faces end users.
    pub sponsor: Option<String>,

    #[serde(deserialize_with = "deserializer::bool", rename = "SRJRCOMBI")]
    pub srjrcombi: bool,

    #[serde(deserialize_with = "deserializer::bool", alias = "FREE")]
    pub free: bool,

    #[serde(deserialize_with = "deserializer::bool")]
    pub dont_show_age_group: bool,

    #[serde(deserialize_with = "deserializer::bool")]
    pub show_entry_times: bool,

    /// Awards configuration for event.
    #[serde(rename = "Prizes")]
    pub awards: Option<Award>,

    #[serde(default)]
    pub round: Option<Round>,

    /// If true the last heat will march in to start block on last heat.
    #[serde(deserialize_with = "deserializer::bool")]
    pub presentation_last_heat: bool,

    /// Undocumented field.
    #[serde(deserialize_with = "deserializer::bool", rename = "Break")]
    pub break_field: bool,

    /// Undocumented field.
    #[serde(deserialize_with = "deserializer::bool")]
    pub prize_ceremony: bool,

    /// Undocumented field.
    #[serde(deserialize_with = "deserializer::bool")]
    pub postpone_heat: bool,

    /// Undocumented field. Might be preferences for time scheduling.
    #[serde(deserialize_with = "deserializer::bool")]
    pub start_after_break: bool,

    /// Undocumented field. Might be preferences for time scheduling.
    pub presentation_time: Option<String>, // four digits with leading zeros

    /// Undocumented field. Might be preferences for time scheduling.
    pub break_time: Option<String>, // five digits with leading zeros

    // Undocumented field. Might be preferences for time scheduling.
    pub prize_ceremony_time: Option<String>, // four digits with leading zeros

    /// Undocumented field. Might be preferences for heat list customization.
    pub prize_ceremony_text: Option<String>,

    /// Undocumented field.
    pub postpone_heat_number: Option<u8>,

    /// Undocumented field.
    pub start_after_break_min: Option<String>,

    /// Undocumented field.
    #[serde(default)]
    pub ses_id: Option<u8>,

    /// Undocumented field.
    pub alt_event_id: u16,

    /// Undocumented field.
    #[serde(default)]
    pub alt_ses_id: Option<u16>,

    /// Undocumented field.
    #[serde(default, deserialize_with = "deserializer::option_bool")]
    pub alt_class_name: Option<bool>,

    /// Undocumented field.
    #[serde(default, deserialize_with = "deserializer::option_bool")]
    pub break_alt: Option<bool>,

    /// Undocumented field
    #[serde(default, deserialize_with = "deserializer::option_bool")]
    pub prize_ceremony_alt: Option<bool>,

    /// Undocumented field.
    pub lenex_event_id: Option<u16>,

    /// Undocumented field.
    pub lenex_event_no: Option<u16>,

    /// Undocumented field.
    pub lenex_event_order: Option<u16>,

    /// Undocumented field.
    #[serde(rename = "DEADLINEDATEWITHDRAWALS")]
    pub deadline_date_withdrawals: Option<u64>, // date

    /// Undocumented field.
    #[serde(rename = "DEADLINETIMEWITHDRAWALS")]
    pub deadline_time_withdrawals: Option<u16>, // four digits time

    /// Undocumented field.
    #[serde(rename = "DEADLINEDATERELAY")]
    pub deadline_date_relay: Option<u64>, // date

    /// Undocumented field.
    #[serde(rename = "DEADLINETIMERELAY")]
    pub deadline_time_relay: Option<u16>, // four digits 24h format
}
