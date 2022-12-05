use std::fmt::{Display, Formatter};

use serde::Deserialize;

use super::error::Error;

/// Competition types
#[derive(Debug)]
pub enum CompetitionType {
    /// International championship
    International = 5,
    /// Unofficial meet
    Unofficial = 6,
    /// Norwegian Championship
    NorwegianChampionship = 8,
    /// Local or regional meet without qualifications for athletes over 9 years of age.
    RegionalWithoutQualification = 15,
}

#[allow(clippy::recursive_format_impl)]
impl Display for CompetitionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match f.align() {
            // Need to split up self into team and individual. self.to_string() will recursively call this function,
            Some(_) => f.pad(&self.to_string()),
            None => match self {
                Self::International => write!(f, "international championship"),
                Self::Unofficial => write!(f, "unofficial"),
                Self::NorwegianChampionship => write!(f, "norwegian championship"),
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
            5 => Ok(Self::International),
            6 => Ok(Self::Unofficial),
            8 => Ok(Self::NorwegianChampionship),
            15 => Ok(Self::RegionalWithoutQualification),
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
            Err(_) => {
                return {
                    Err(serde::de::Error::unknown_variant(
                        &deserialized_value,
                        &["parsable integer"],
                    ))
                };
            }
        };

        match Self::try_from(number) {
            Ok(distance) => Ok(distance),
            Err(_) => Err(serde::de::Error::unknown_variant(
                &deserialized_value,
                &["DEFAULT", "MEDALS", "NO", "3"],
            )),
        }
    }
}

//pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<CompetitionType>, D::Error>
//where
//    D: serde::de::Deserializer<'de>,
//{
//    const EXPECTED: &str = "one of [5, 6, 8, 15]";
//    let deserialized_value: String = Deserialize::deserialize(deserializer)?;
//
//    let number = match deserialized_value.parse::<u8>() {
//        Ok(number) => number,
//        Err(err) => {
//            return Err(serde::de::Error::invalid_value(
//                serde::de::Unexpected::Str(&err.to_string()),
//                &EXPECTED,
//            ));
//        }
//    };
//
//    match CompetitionType::from_u8(number) {
//        Ok(distance) => Ok(Some(distance)),
//        Err(_) => Err(serde::de::Error::unknown_variant(
//            &deserialized_value,
//            &["DEFAULT", "MEDALS", "NO", "3"],
//        )),
//    }
//}
