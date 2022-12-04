use super::{
    award::Award,
    deserializer,
    distance::Distance,
    gender_group::GenderGroup,
    pool_length::PoolLength,
    round::{self, Round},
    sorting::Sorting,
    style::Style,
};
use chrono::NaiveDate;
use serde::Deserialize;
use std::time::Duration;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[allow(clippy::struct_excessive_bools)]
#[serde(rename_all = "PascalCase")]
pub struct Event {
    /// event number. Starts with 1.
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

    /// Undocumented field.
    #[serde(deserialize_with = "deserializer::bool")]
    pub senior: bool,

    /// Undocumented field.
    #[serde(deserialize_with = "deserializer::bool")]
    pub junior: bool,

    /// Undocumented field.
    #[serde(deserialize_with = "deserializer::bool")]
    pub junior_older: bool,

    /// Undocumented field.
    #[serde(deserialize_with = "deserializer::bool")]
    pub junior_younger: bool,

    /// Undocumented field.
    #[serde(deserialize_with = "deserializer::option_year", default)]
    pub youngest: Option<datetime::Year>,

    /// Undocumented field.
    #[serde(deserialize_with = "deserializer::option_year", default)]
    pub oldest: Option<datetime::Year>,

    /// Undocumented field.
    pub event_pool_length: PoolLength,

    /// Starting date for the meet
    #[serde(deserialize_with = "deserializer::date")]
    pub date: NaiveDate,

    /// Undocumented field.
    #[serde(
        default,
        rename = "QualLongCourse",
        deserialize_with = "deserializer::option_duration"
    )]
    pub qualification_time_long_course: Option<Duration>,

    /// Undocumented field.
    #[serde(
        default,
        rename = "QualShortCourse",
        deserialize_with = "deserializer::option_duration"
    )]
    pub qualification_time_short_course: Option<Duration>,

    /// Undocumented field.
    pub sorting: Sorting,

    /// No qualification for handicap.
    #[serde(deserialize_with = "deserializer::bool", rename = "NoQualHcEvent")]
    pub no_qualification_for_handicap: bool,

    /// Undocumented field.
    #[serde(deserialize_with = "deserializer::bool", rename = "Webheat")]
    pub web_heat: bool,

    /// Event sponsor text. Hint for other applications that might use this information if it faces end users.
    pub sponsor: Option<String>,

    /// Undocumented field.
    #[serde(deserialize_with = "deserializer::bool", rename = "SRJRCOMBI")]
    pub srjrcombi: bool,

    /// Undocumented field,
    #[serde(deserialize_with = "deserializer::bool", alias = "FREE")]
    pub free: bool,

    /// Undocumented field.
    #[serde(deserialize_with = "deserializer::bool")]
    pub dont_show_age_group: bool,

    /// Undocumented field.
    #[serde(deserialize_with = "deserializer::bool")]
    pub show_entry_times: bool,

    /// Awards configuration for event.
    #[serde(rename = "Prizes")]
    pub awards: Option<Award>,

    #[serde(deserialize_with = "round::deserialize")]
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
