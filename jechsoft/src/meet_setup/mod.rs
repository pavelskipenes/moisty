//! Parser for `meet_setup.xml` file

use self::meet_info::MeetInfo;
use serde::Deserialize;

mod age_group;
mod australian_rank;
mod australian_world_record;
mod award;
pub mod class;
mod competition_type;
mod deserializer;
pub mod distance;
mod event;
pub mod gender_class;
pub mod gender_group;
mod handicap;
mod individual_distance;
mod junior;
pub mod meet;
mod meet_info;
mod person;
mod pool_category;
pub mod pool_length;
mod qualification;
mod qualification_set;
mod round;
mod session;
mod sorting;
pub mod style;
mod team_distance;
mod touch_pad_set;
pub mod utils;

#[derive(Deserialize)]
pub struct Entries {
    #[serde(rename = "strc_stevneoppsett")]
    pub meet_setup_entries: Vec<MeetInfo>,
}
