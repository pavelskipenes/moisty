use super::deserializer;
use chrono::NaiveDate;
use reqwest::Url;
use serde::Deserialize;

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
