//! Parser for `meet_setup.xml` file

use self::meet_info::MeetInfo;
use serde::Deserialize;

mod age_group;
mod australian_world_record;
mod award;
mod class;
mod competition_type;
mod deserializer;
mod distance;
mod error;
mod event;
mod gender_group;
mod handicap;
mod junior;
pub mod meet;
mod meet_info;
mod person;
mod pool_category;
mod pool_length;
mod qualification;
mod qualification_set;
mod round;
mod session;
mod sorting;
mod style;
mod touch_pad_set;
pub mod utils;

#[derive(Deserialize)]
pub struct Entries {
    #[serde(rename = "strc_stevneoppsett")]
    pub meet_setup_entries: Vec<MeetInfo>,
}
