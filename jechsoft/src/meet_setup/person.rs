extern crate gregorian;
use self::gregorian::Year;
use super::gender_group::GenderGroup;
extern crate serde;
use self::serde::Deserialize;

/// Simple person struct
#[derive(Debug, Deserialize, Clone)]
pub struct Person {
    /// Surname.
    #[serde(rename = "LastName")]
    pub surname: String,

    /// First name.
    #[serde(rename = "FirstName")]
    pub name: String,

    /// Gender.
    #[serde(rename = "Sex")]
    pub gender: GenderGroup,

    /// Birth date.
    #[serde(
        rename = "BirthDate",
        deserialize_with = "deserialize_birth_date",
        default
    )]
    pub birth_date: Option<Year>,

    /// Club.
    #[serde(rename = "Club")]
    pub club: String,
}

/// Deserialize bith date in Person struct
fn deserialize_birth_date<'de, D>(deserializer: D) -> Result<Option<Year>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::de::Deserialize::deserialize(deserializer)?;

    if s == "SR" {
        return Ok(None);
    }
    if s == "JR" {
        return Ok(None);
    }

    match s.parse::<i16>() {
        Ok(year) => Ok(Some(Year::new(year))),
        Err(why) => Err(serde::de::Error::custom(why.to_string()))?,
    }
}
