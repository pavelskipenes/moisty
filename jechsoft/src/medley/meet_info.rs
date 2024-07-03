extern crate chrono;
extern crate reqwest;
extern crate serde;

use self::chrono::NaiveDate;
use self::reqwest::Url;
use super::serde::Deserialize;
use meet_setup::deserializer;

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
    /// Generate a consistent filename from it's meet name
    // Assumed that this method will yield exactly the same result as `Meet::get_filename(&self)`
    // It would probably be wise to combine those two functions together into one in the future
    // It would also be nice to use more information and get more consistent meet name across
    // different years and so on.
    // TODO: consider using nsf_medley_id to index files
    // pros:
    // - unique for norwegian meets
    // cons:
    // - optional to specify, specially for unofficial meets
    // - international meets might not have it
    #[must_use]
    pub fn get_filename(&self) -> String {
        self.name.replace(' ', "_").to_lowercase()
    }
}
