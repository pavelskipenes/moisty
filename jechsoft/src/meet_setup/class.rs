use super::{error::Error, handicap::Handicap, junior::Junior};
use chrono::{Datelike, Local};
use datetime::Year;
use serde::Deserialize;

/// Each Athlete is a member of one class. Athletes that are in the same Class compete against each other. `Athlete`s cannot compete against each other across `Class`es An Athlete can be a member of only one `Class`. `Athlete`s `Class` is dependent on his/her age.
/// > TODO: Athletes might maybe be members of multiple `Handicap` `Class`es. Needs confirmation.

#[derive(Debug)]
pub enum Class {
    /// Athletes that is older than 19 years old.
    Senior,
    /// For Athletes age 9 and 19. Junior team relays is not bound by year.
    Junior(Option<Year>), // Team relays is not bound by year
    /// For disabled Athletes
    Handicap(Handicap),
}

impl Class {
    /// Try to "unwrap" to a `Junior` variant
    /// # Errors
    /// - returns `Error::NotAJuniorAge` if `self` is not in valid `Junior` range.
    /// - returns `Error::TeamRelaysDoesNotHaveJuniorClassGroup` if `self` is a junior team without
    /// a class year.
    /// - returns `Error::NotAJuniorVariant` if `self` is not a Junior variant. So either a `Senior` or a `Handicap`
    pub fn try_into_junior_class(self, meet_year: Year) -> Result<Junior, Error> {
        let meet_year = meet_year.abs();
        if let Self::Junior(year) = self {
            year.map_or(Err(Error::TeamRelaysDoesNotHaveJuniorClassGroup), |year| {
                Junior::try_from(meet_year.abs_diff(year.abs()))
                    .map_or(Err(Error::NotAJuniorAge), Ok)
            })
        } else {
            Err(Error::NotAJuniorVariant)
        }
    }
}

impl<'de> Deserialize<'de> for Class {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const EXPECTED: &str = "'SR' | 'Sx' | 'SMx' | 'SBx' | 'JR' where x is a number between 1 and 15 or a four digit number representing a year not further away than 100 years from current year";
        let deserialized_value: String = Deserialize::deserialize(deserializer)?;
        if deserialized_value == "SR" {
            return Ok(Self::Senior);
        }
        if deserialized_value == "JR" {
            return Ok(Self::Junior(None));
        }

        let maybe_first_character = deserialized_value.chars().next();
        match maybe_first_character {
            None => Err(serde::de::Error::custom("input string too short")),
            Some(first_character) => match first_character {
                'S' | 's' => {
                    let handicap = Handicap::try_from(deserialized_value.as_str()).map_err(
                        |_err| -> D::Error {
                            serde::de::Error::invalid_value(
                                serde::de::Unexpected::Str(&deserialized_value),
                                &EXPECTED,
                            )
                        },
                    )?;
                    Ok(Self::Handicap(handicap))
                }
                '0'..='9' => {
                    // might be a valid year
                    deserialized_value.parse::<i64>().map_or_else(
                        |_parse_error| {
                            Err(serde::de::Error::invalid_value(
                                serde::de::Unexpected::Str(&deserialized_value),
                                &EXPECTED,
                            ))
                        },
                        |year| {
                            let year = Year(year);
                            if year.abs_diff(Local::now().year().into()) > 100 {
                                Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&deserialized_value),
                                    &EXPECTED,
                                ))
                            } else {
                                Ok(Self::Junior(Some(year)))
                            }
                        },
                    )
                }
                _ => Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(&deserialized_value),
                    &EXPECTED,
                )),
            },
        }
    }
}
