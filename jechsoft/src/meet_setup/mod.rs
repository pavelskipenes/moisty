//! Parser for `meet_setup.xml` file

use self::meet_info::MeetInfo;
use serde::Deserialize;

mod age_group;
mod australian_world_record;
mod award;
mod competition_type;
mod deserializer;
mod error;
mod event;
mod handicap;
mod junior;
mod meet_info;
mod person;
mod pool_category;
mod qualification_set;
mod qualification;
mod round;
mod session;
mod sorting;
mod touch_pad_set;
pub mod class;
pub mod distance;
pub mod gender_class;
pub mod gender_group;
pub mod meet;
pub mod pool_length;
pub mod style;
pub mod utils;

#[derive(Deserialize)]
pub struct Entries {
    #[serde(rename = "strc_stevneoppsett")]
    pub meet_setup_entries: Vec<MeetInfo>,
}
