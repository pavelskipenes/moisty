use super::deserializer;
use chrono::NaiveDate;
use reqwest::Url;
use serde::Deserialize;

/// Meet info structure parses output from a url endpoint used by "JechSoft Victoria"
/// for searching upcoming meets and download them into it's database without downloading
/// it manually from [https://medley.no](https://medley.no)
#[derive(Deserialize, Debug)]
#[serde(rename = "strc_stevneoppsett")]
pub struct MeetInfo {
    /// Meet name
    #[serde(rename = "stevnenavn")]
    pub name: String,

    /// Starting date of the meet
    #[serde(rename = "fradato", deserialize_with = "deserializer::date")]
    pub date_start: NaiveDate,

    /// Ending date of the meet
    #[serde(rename = "tildato", deserialize_with = "deserializer::date")]
    pub date_end: NaiveDate,

    /// Host club name
    #[serde(rename = "arrangor")]
    pub host: String,

    /// Norwegian swimming federation meet id
    #[serde(rename = "nsfstevneid")]
    pub id: u32,

    /// Link to Jechsoft Victoria meet configuration file `meetsetup.xml`
    #[serde(rename = "xmllink")]
    pub config_xml: Url,
}
