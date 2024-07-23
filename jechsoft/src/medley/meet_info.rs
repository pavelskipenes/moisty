extern crate chrono;
extern crate reqwest;
extern crate serde;

use super::serde::Deserialize;
use crate::meet_setup::deserializer;

use self::chrono::NaiveDate;
use self::reqwest::Url;

/// Meet info structure parses output from a url endpoint used by `JechSoft Victoria`
/// for searching upcoming meets and download them into it's database without downloading
/// it manually from [https://medley.no](https://medley.no)
/// Worth noting is that the endpoint at medley does not include last modification date. Result of
/// this is that meets need to be redownloaded all the time in case they are different.
#[derive(Deserialize, Debug)]
#[serde(rename = "strc_stevneoppsett")]
pub struct MeetInfo {
    /// Meet name
    #[serde(rename = "stevnenavn")]
    pub name: String,

    /// Starting date of the meet
    #[serde(rename = "fradato", deserialize_with = "deserializer::date")]
    pub date_start: NaiveDate,

    /// Last date of the meet
    #[serde(rename = "tildato", deserialize_with = "deserializer::date")]
    pub date_end: NaiveDate,

    /// Host. Meets are usually organized by swimming clubs and therefore swimming clubs usually
    /// appear here
    #[serde(rename = "arrangor")]
    pub host: String,

    /// Norwegian swimming federation meet id.
    /// Required for all official meets in Norway.
    /// Unofficial meets do not require meet id.
    /// represented as 11 digit numerical digit with leading zeros.
    /// # Example:
    /// "00000012345"
    #[serde(rename = "nsfstevneid")]
    pub id: u32,

    /// Link to Jechsoft Victoria meet configuration file `meetsetup.xml`
    #[serde(rename = "xmllink")]
    pub meet_setup: Url,
}

impl MeetInfo {
    /// Generate a file name for each meet.
    /// File names are generated using medley id because they are unique.
    /// pros:
    /// - unique for norwegian meets
    ///   cons:
    /// - optional to specify, specially for unofficial meets
    /// - international meets might not have it
    /// - non memebers of NSF will not have this id
    #[must_use]
    pub fn get_filename(&self) -> String {
        format!("{:0>11}", self.id)
    }
}
