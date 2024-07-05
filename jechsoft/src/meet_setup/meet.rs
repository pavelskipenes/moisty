extern crate chrono;
extern crate encoding;
extern crate gregorian;
extern crate reqwest;
extern crate serde;
extern crate serde_email;
extern crate serde_xml_rs;

use self::chrono::NaiveDate;
use self::gregorian::Year;
use self::reqwest::Url;
use self::serde::Deserialize;
use self::serde_email::Email;
use super::{
    age_group::DefinedAgeGroups, australian_rank::AustralianRank,
    australian_world_record::AustralianWorldRecord, award::Award,
    competition_type::CompetitionType, deserializer, distance::Distance, event::Event,
    person::Person, pool_category::PoolCategory, qualification_set::QualificationSet,
    session::Session, touch_pad_set::TouchPadSet,
};
use std::error::Error;
use std::io::BufReader;
use std::{fs::File, path::Path};

/// `MeetConfig` is a rust structure that represents `meetsetup.xml` file used by Jechsoft Victoria.
/// This file contains settings for a meet. This structure extract those fields and parses then
/// into usable rust structures with simple type validation. This file contains only the meet
/// settings, `meetresults.xml` contains data related to enrollment and results of athletes on the
/// same meet.
/// TODOs:
/// - [ ] Refactor pub products: Vec<(String, Price)>. Look at #[serde(flatten)]
/// - [ ] Use money package for deserializing currency like [rusty money](https://docs.rs/rusty-money/latest/rusty_money/)
/// - [ ] Group together configuration for heat list generation
/// - [ ] Group together configuration for scheduling
/// - [ ] Add methods for non data type validation that will pass type validation
#[derive(Deserialize, Debug)]
#[serde(rename = "MeetSetUp", rename_all = "PascalCase")]
#[allow(clippy::struct_excessive_bools)]
#[serde(deny_unknown_fields)]
pub struct Meet {
    /// Jechsoft meet config version
    pub nsf_version: String,

    /// Parsed config was generated by this software
    pub creator: String,

    /// Norwegian swimming federation meet id (nsfid). 10 digit id with leading zeros
    #[serde(rename = "NSFMeetId")]
    pub nsf_meet_id: Option<u32>,

    /// Meet name.
    #[serde(rename = "MeetName")]
    pub name: String,

    /// readable representation of the date.
    #[serde(rename = "MeetDate")]
    pub date: String,

    /// Location of the meet.
    #[serde(rename = "MeetPlace")]
    pub location: String,

    pub pool_category: PoolCategory,

    /// Pool length
    pub pool_length: Distance,

    pub start_with_lane: Option<u8>,

    /// Number of lanes in the competition.
    pub lanes: u8,

    /// Enrollment price in NOK for one enrollment entry for one athlete.
    pub individual_price: u16,

    /// Enrollment price in NOK for one enrollment entry for one relay team.
    pub team_price: u16,

    pub individual_price2: u16,

    pub team_price2: u16,

    /// Enrollment price in NOK for athletes that qualify for paying one price for unlimited entries.
    pub one_price_all: u16,

    // Athlete with this birth year pays once for unlimited starts.
    #[serde(
        rename = "OnePriceAllClasses",
        deserialize_with = "deserializer::one_price_all_class",
        default
    )]
    pub birth_years_pay_once: Option<Vec<Year>>,

    #[serde(deserialize_with = "deserializer::bool")]
    pub australian_model: bool,

    pub australian_rank: AustralianRank,

    pub australian_world_record: AustralianWorldRecord,

    /// Merge all handicap classes into one single handicap class.
    #[serde(rename = "HCSingleAgeGroup", deserialize_with = "deserializer::bool")]
    pub hc_single_age_group: bool,

    /// Female athletes born this year or earlier (older athletes) are competing in senior class.
    #[serde(default, deserialize_with = "deserializer::option_year")]
    pub women_senior: Option<Year>,

    #[serde(default)]
    pub extra_time_backstroke: Option<u8>,

    /// Male athletes born this year or earlier (older athletes) are competing in senior class.
    #[serde(
        default,
        alias = "menSenior",
        deserialize_with = "deserializer::option_year"
    )]
    pub men_senior: Option<Year>,

    // TODO: men_junior: Vec<Year>;
    // TODO: women_junior: Vec<Year>;
    #[serde(default, deserialize_with = "deserializer::option_year")]
    pub women_junior: Option<Year>,

    #[serde(default, deserialize_with = "deserializer::option_year")]
    pub men_junior: Option<Year>,

    #[serde(default, deserialize_with = "deserializer::option_year")]
    pub women_junior2: Option<Year>,

    #[serde(default, deserialize_with = "deserializer::option_year")]
    pub men_junior2: Option<Year>,

    #[serde(default, deserialize_with = "deserializer::option_year")]
    pub women_youngest_final: Option<Year>,

    #[serde(default, deserialize_with = "deserializer::option_year")]
    pub men_youngest_final: Option<Year>,

    /// If true then the meet is primarily a masters meet.
    #[serde(deserialize_with = "deserializer::bool")]
    pub primary_masters: bool,

    /// Enrollment with personal best has to be no later than this date.
    #[serde(deserialize_with = "deserializer::date")]
    pub final_entry_date: NaiveDate,

    /// Enrollment with personal best has to be no older than this date.
    #[serde(deserialize_with = "deserializer::date")]
    pub first_entry_date: NaiveDate,

    /// Last enrollment date.
    #[serde(deserialize_with = "deserializer::date")]
    pub last_entry_date: NaiveDate,

    /// If true, qualifications don't apply for handicapped athletes.
    #[serde(rename = "NoQualHC", deserialize_with = "deserializer::bool")]
    pub no_qual_hc: bool,

    /// Date meet start.
    #[serde(
        default,
        rename = "StartDate",
        deserialize_with = "deserializer::option_date"
    )]
    pub date_start: Option<NaiveDate>,

    /// Date meet end.
    #[serde(
        default,
        rename = "EndDate",
        deserialize_with = "deserializer::option_date"
    )]
    pub date_end: Option<NaiveDate>,

    /// Host club.
    pub host_club: Option<String>,

    /// Organization number of host club.
    /// Samples:
    /// - "GR01040025450"
    /// - "GR03010168450"
    /// - "GR04270045450"
    /// - "GR06170001450"
    /// - "GR07010020450"
    /// - "GR07060028450"
    /// - "GR11030050450"
    /// - "GR15340002450"
    /// - "GR18330025450"
    ///
    /// Starts always with "GR" and then 11 numbers with leading zeros.
    pub host_club_organization_no: Option<String>, // TODO: create struct for this type

    /// Competition type specifies what kind of competition it is and what rules apply for it.
    /// A single competiton type will have some rules specifying whether certain age groups can
    /// compete and whether there are any entrollment qualifications for the meet.
    pub competition_type_id: CompetitionType,

    pub community: Option<String>,

    /// Human readable string representation of the meet. Might be redundant because of `competition_type_id`.
    pub competition_type: String,

    /// Url to where results can be found.
    #[serde(default, rename = "ResultWebaddress")]
    pub result_web_address: Option<Url>,

    /// homepage
    #[serde(default, rename = "Homepage")]
    pub home_page: Option<Url>,

    /// enrollment email address
    // BUG: deserializes to None all the time
    #[serde(default, alias = "MailPameldinger", alias = "EntryMail")]
    pub entry_email: Option<Email>,

    /// Payment information for clubs.
    #[serde(default)]
    pub pay_account: String,

    #[serde(default, deserialize_with = "deserializer::option_bool")]
    pub general_senior: Option<bool>,

    #[serde(default, deserialize_with = "deserializer::option_bool")]
    pub general_junior: Option<bool>,

    #[serde(
        default,
        rename = "GeneralHC",
        deserialize_with = "deserializer::option_bool"
    )]
    pub general_hc: Option<bool>,

    #[serde(default, rename = "PoolLengthStartHeat")]
    pub pool_length_start_heat: Option<String>,

    #[serde(default, rename = "LCMEntrytimes")]
    pub lcm_entry_times: Option<String>,

    #[serde(default, rename = "SCMEntrytimesIfLCMDoesNotExist")]
    pub scm_entry_times_if_lcm_does_not_exists: Option<String>,

    #[serde(default, rename = "SortLCMBeforeSCM")]
    pub sort_lcm_before_scm: Option<String>,

    #[serde(default, deserialize_with = "deserializer::option_bool")]
    pub general_masters: Option<bool>,

    #[serde(default, deserialize_with = "deserializer::option_bool")]
    pub no_pool: Option<bool>,

    /// If true then the meet has been cancelled.
    #[serde(deserialize_with = "deserializer::bool")]
    pub cancelled: bool,

    /// Optional info box in meet configuration.
    #[serde(default)]
    pub info: Option<String>,

    #[serde(deserialize_with = "deserializer::bool")]
    pub write_country: bool,

    /// A configuration that controls whether records for the event should be printed in the heat
    /// lists.
    // TODO: group this setting into heat list congiguration
    #[serde(rename = "RecordsInHeatlist", deserialize_with = "deserializer::bool")]
    pub records_in_heat_list: bool,

    #[serde(default, deserialize_with = "deserializer::option_bool")]
    pub write_first_lap: Option<bool>,

    /// A configuration that controls whether page number should be printed in heat lists.
    // TODO: group this setting into heat list congiguration
    #[serde(
        rename = "PageNumberInHeatlist",
        deserialize_with = "deserializer::option_bool"
    )]
    pub page_number_in_heat_list: Option<bool>,

    #[serde(
        default,
        deserialize_with = "deserializer::option_bool",
        alias = "Skriv1etappe"
    )]
    pub write_first_stage: Option<bool>,

    #[serde(deserialize_with = "deserializer::bool")]
    pub use_group_text: bool,

    #[serde(deserialize_with = "deserializer::bool")]
    pub show_time_schedule: bool,

    #[serde(deserialize_with = "deserializer::bool")]
    pub show_time_only_heat_one: bool,

    #[serde(deserialize_with = "deserializer::bool")]
    pub show_heat_text: bool,

    /// Touch pad configuration of the meet.
    #[serde(rename = "Touchpads")]
    pub touch_pads: TouchPadSet,

    #[serde(deserialize_with = "deserializer::bool")]
    pub write_other_prices: bool,

    /// If true non of the results will be reported to national record database.
    #[serde(deserialize_with = "deserializer::bool")]
    pub unofficial: bool,

    // TODO: `products: HashSet<String, Price>`
    #[serde(default)]
    pub other_payment1: Option<String>,

    #[serde(default)]
    pub other_price1: Option<u16>,

    #[serde(default)]
    pub other_payment2: Option<String>,

    #[serde(default)]
    pub other_price2: Option<u16>,

    #[serde(default)]
    pub other_payment3: Option<String>,

    #[serde(default)]
    pub other_price3: Option<u16>,

    #[serde(default)]
    pub other_payment4: Option<String>,

    #[serde(default)]
    pub other_price4: Option<u16>,

    #[serde(default)]
    pub other_payment5: Option<String>,

    #[serde(default)]
    pub other_price5: Option<u16>,

    #[serde(default)]
    pub other_payment6: Option<String>,

    #[serde(default)]
    pub other_price6: Option<u16>,

    #[serde(default)]
    pub other_payment7: Option<String>,

    #[serde(default)]
    pub other_price7: Option<u16>,

    #[serde(default)]
    pub other_payment8: Option<String>,

    #[serde(default)]
    pub other_price8: Option<u16>,

    #[serde(deserialize_with = "deserializer::bool")]
    pub write_date_time: bool,

    pub header: Option<String>,

    pub footer: Option<String>,

    /// Default award configuration for the meet. This value is used if `this.events.awards = Award::Default`.
    #[serde(rename = "Prizes")]
    pub awards: Option<Award>,

    #[serde(default, deserialize_with = "deserializer::option_bool")]
    pub start_on_minute: Option<bool>,

    pub time_between: Option<u16>,

    pub extra_time: Option<u16>,

    /// List of sessions in the meet. A session is a set of continuos heats without breaks.
    #[serde(deserialize_with = "deserializer::session")]
    pub sessions: Vec<Session>,

    /// List of qualification requirements for enrollment.
    #[serde(default)]
    pub qualification_set: Option<QualificationSet>,

    /// Host representative for managing enrollment.
    #[serde(rename = "EntryManager")]
    pub entry_manager: Option<Person>,

    #[serde(default, rename = "DefinedAgeGroups")]
    pub age_groups: Option<DefinedAgeGroups>,

    /// Host representative for managing the meet. Also called "meet leader".
    #[serde(rename = "CompetitionManager")]
    pub competition_manager: Option<Person>,

    /// Events
    #[serde(deserialize_with = "deserializer::event")]
    pub events: Vec<Event>,
}

impl Meet {
    ///#  Errors
    /// returns Error if:
    /// - `local_xml_file` cannot be opened.
    /// - deserialization fails
    pub fn try_from(local_xml_file: &Path) -> Result<Self, Box<dyn Error>> {
        let file = File::open(local_xml_file)?;
        let reader = BufReader::new(file);
        let meet: Self = serde_xml_rs::de::from_reader(reader)?;

        Ok(meet)
    }

    // Here we assume that we get the exactly the same name as `MeetInfo::get_filename(&self)`
    #[must_use]
    pub fn get_filename(&self) -> String {
        self.name.replace(' ', "_").to_lowercase()
    }
}
