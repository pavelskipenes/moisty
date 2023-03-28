use super::error::Error;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

/// Competition types
#[derive(Debug)]
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
    NonNorwegianMeet = 19,
}
const EXPECTED_DESERIALIZER_INPUT: [&str; 7]= ["3", "5", "6", "8", "15", "16", "19"];

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
                Self::NationalMeetWithAthletesFromForeginNations => write!(f, "national meet with athletes from foregin nations"),
                Self::NonNorwegianMeet => write!(f, "non Norwegian meet"),
                // Self::AgeClassChempionship => write!(f, "age class championship"),
                Self::RegionalWithoutQualification => {
                    write!(f, "regional without qualifications")
                }
            },
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
            19 => Ok(Self::NonNorwegianMeet),
            // ?? => Ok(Self::AgeClassChempionship),
            _ => Err(Error::CompetitionTypeIdDoesNotExists),
        }
    }
}

impl<'de> Deserialize<'de> for CompetitionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let deserialized_value: String = Deserialize::deserialize(deserializer)?;

        let number = match deserialized_value.parse::<u8>() {
            Ok(number) => number,
            Err(_) => Err(serde::de::Error::unknown_variant(
                &deserialized_value,
                &["parsable integer"],
            ))?,
        };

        Self::try_from(number).map_or_else(
            |_| {
                Err(serde::de::Error::unknown_variant(
                    &deserialized_value,
                     &EXPECTED_DESERIALIZER_INPUT,
                ))
            },
            Ok,
        )
    }
}

// pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<CompetitionType>, D::Error>
// where
//    D: serde::de::Deserializer<'de>,
// {
//    const EXPECTED: &str = "one of [5, 6, 8, 15]";
//    let deserialized_value: String = Deserialize::deserialize(deserializer)?;

//    let number = match deserialized_value.parse::<u8>() {
//        Ok(number) => number,
//        Err(err) => {
//            return Err(serde::de::Error::invalid_value(
//                serde::de::Unexpected::Str(&err.to_string()),
//                &EXPECTED,
//            ));
//        }
//    };

//    CompetitionType::try_from(number).map_or_else(|_| Err(serde::de::Error::unknown_variant(
//            &deserialized_value,
//            &EXPECTED_DESERIALIZER_INPUT,
//        )), |distance| Ok(Some(distance)))
// }
