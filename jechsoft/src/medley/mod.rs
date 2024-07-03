extern crate serde;
use self::meet_info::MeetInfo;
use self::serde::Deserialize;

pub mod meet_info;
pub mod utils;

#[derive(Deserialize)]
pub struct Entries {
    #[serde(rename = "strc_stevneoppsett")]
    pub meet_setup_entries: Vec<MeetInfo>,
}
