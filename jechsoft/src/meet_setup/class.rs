use chrono::{Datelike, Local};
use datetime::Year;
use serde::Deserialize;

use super::{error::Error, handicap::Handicap, junior::Junior};

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
    pub fn get_class_group(&self, meet_year: Year) -> Result<Junior, Error> {
        let meet_year = meet_year.abs();
        if let Self::Junior(year) = self {
            year.map_or(Err(Error::TeamRelaysDoesNotHaveJuniorClassGroup), |year| {
                match Junior::try_from(meet_year.abs_diff(year.abs())) {
                    Ok(junior_class) => Ok(junior_class),
                    Err(_) => Err(Error::NotAJuniorAge),
                }
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
                    let handicap = Handicap::from_str(deserialized_value.as_str()).map_err(
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
                    match deserialized_value.parse::<i64>() {
                        // TODO: make sure the created year is within reasonable range
                        Ok(year) => {
                            let year = Year(year);
                            if year.abs_diff(Local::now().year().into()) > 100 {
                                Err(serde::de::Error::invalid_value(
                                    serde::de::Unexpected::Str(&deserialized_value),
                                    &EXPECTED,
                                ))
                            } else {
                                Ok(Self::Junior(Some(year)))
                            }
                        }
                        Err(_) => Err(serde::de::Error::invalid_value(
                            serde::de::Unexpected::Str(&deserialized_value),
                            &EXPECTED,
                        )),
                    }
                }
                _ => Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(&deserialized_value),
                    &EXPECTED,
                )),
            },
        }
    }
}
