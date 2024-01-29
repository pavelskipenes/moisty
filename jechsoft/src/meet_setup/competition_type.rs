use serde::Deserialize;
use std::{
    convert::TryFrom,
    fmt::{Display, Formatter},
};

/// Competition types
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CompetitionType {
    /// Meet for mentally disabled athletes. See `Class::(style_group: Handicap{StyleGroup::FreestyleBackstrokeButterfly, dissability_type: 14})`
    MentallyDisabledMeet = 3,
    /// International championship
    International = 5,
    /// National meet with athletes from other nations
    NationalMeetWithAthletesFromForeginNations = 4,
    /// Unofficial meet
    Unofficial = 6,
    /// Norwegian Championship
    NorwegianChampionship = 8,
    /// Local or regional meet without qualifications for athletes over 9 years of age.
    RegionalWithoutQualification = 15,
    /// Regional Age Class Meet / LÅMØ
    RegionalAgeGroupMeet = 16,
    // AgeClassChampionship / ÅM
    // AgeClassChempionship = todo!(),
    //
    /// "Krets/Regionstevne"
    DistrictRegionalMeet = 18,

    NonNorwegianMeet = 19,
}

#[allow(clippy::recursive_format_impl)]
impl Display for CompetitionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match f.align() {
            // Need to split up self into team and individual. self.to_string() will recursively call this function,
            Some(_) => f.pad(&self.to_string()),
            None => match self {
                Self::MentallyDisabledMeet => write!(f, "meet for mentally disabled athletes"),
                Self::International => write!(f, "international championship"),
                Self::Unofficial => write!(f, "unofficial"),
                Self::NorwegianChampionship => write!(f, "Norwegian championship"),
                Self::RegionalAgeGroupMeet => write!(f, "regional age group meet"),
                Self::NationalMeetWithAthletesFromForeginNations => {
                    write!(f, "national meet with athletes from foregin nations")
                }
                Self::NonNorwegianMeet => write!(f, "non Norwegian meet"),
                // Self::AgeClassChempionship => write!(f, "age class championship"),
                Self::RegionalWithoutQualification => {
                    write!(f, "regional without qualifications")
                }
                Self::DistrictRegionalMeet => write!(f, "district / regional meet"),
            },
        }
    }
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
    CompetitionTypeIdDoesNotExists,
    Deseriaize(std::num::ParseIntError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CompetitionTypeIdDoesNotExists => write!(f, "competiton type id does not exists"),
            Self::Deseriaize(parse_int_error) => {
                write!(f, "deseriaization error: {parse_int_error}")
            }
        }
    }
}

impl TryFrom<u8> for CompetitionType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            3 => Ok(Self::MentallyDisabledMeet),
            4 => Ok(Self::NationalMeetWithAthletesFromForeginNations),
            5 => Ok(Self::International),
            6 => Ok(Self::Unofficial),
            8 => Ok(Self::NorwegianChampionship),
            15 => Ok(Self::RegionalWithoutQualification),
            16 => Ok(Self::RegionalAgeGroupMeet),
            18 => Ok(Self::DistrictRegionalMeet),
            19 => Ok(Self::NonNorwegianMeet),
            // ?? => Ok(Self::AgeClassChempionship),
            _ => Err(Error::CompetitionTypeIdDoesNotExists),
        }
    }
}

impl TryFrom<&str> for CompetitionType {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.parse::<u8>() {
            Ok(number) => Self::try_from(number),
            Err(parse_int_error) => Err(Error::Deseriaize(parse_int_error)),
        }
    }
}

impl<'de> Deserialize<'de> for CompetitionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const EXPECTED_DESERIALIZER_INPUT: [&str; 9] =
            ["3", "4", "5", "6", "8", "15", "16", "18", "19"];

        let deserialized_value: String = Deserialize::deserialize(deserializer)?;
        let parse_error: D::Error =
            serde::de::Error::unknown_variant(&deserialized_value, &EXPECTED_DESERIALIZER_INPUT);

        Self::try_from(deserialized_value.as_ref()).map_err(|_| parse_error)
    }
}
