use super::handicap::Handicap;
use chrono::{Datelike, Local};
use gregorian::Year;
use serde::Deserialize;

/// Each Athlete is a member of one class. Athletes that are in the same Class compete against each other. `Athlete`s cannot compete against each other across `Class`es An Athlete can be a member of only one `Class`. `Athlete`s `Class` is dependent on his/her age.
/// > TODO: Athletes might maybe be members of multiple `Handicap` `Class`es. Needs confirmation.

#[derive(Debug, Copy, Clone)]
pub enum Class {
    /// Athletes that is older than 19 years old.
    Senior,
    /// For Athletes age 9 and 19. Junior team relays is not bound by year.
    Junior(Option<Year>), // Team relays is not bound by year
    /// For disabled Athletes
    Handicap(Handicap),
}

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum Error {}

impl<'de> Deserialize<'de> for Class {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const EXPECTED: &str = "'SR' | 'Sx' | 'SMx' | 'SBx' | 'JR' where x is a number between 1 and 15 or a four digit number representing a year not further away than 100 years from current year";
        let deserialized_value: String = Deserialize::deserialize(deserializer)?;
        
        // TODO: D::Error does not implement clone so this shit needs to be created explicitly.
        let error: D::Error = serde::de::Error::invalid_value(
            serde::de::Unexpected::Str(&deserialized_value),
            &EXPECTED,
        );
        let error2: D::Error = serde::de::Error::invalid_value(
            serde::de::Unexpected::Str(&deserialized_value),
            &EXPECTED,
        );
        let error3: D::Error = serde::de::Error::invalid_value(
            serde::de::Unexpected::Str(&deserialized_value),
            &EXPECTED,
        );

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
                    let handicap =
                        Handicap::try_from(deserialized_value.as_str()).map_err(|_err| error)?;
                    Ok(Self::Handicap(handicap))
                }
                '0'..='9' => {
                    // might be a valid year
                    deserialized_value.parse::<i16>().map_or_else(
                        |_parse_error| Err(error2),
                        |year| {
                            let year = Year::new(year);

                            let unexpected = serde::de::Unexpected::Str(&deserialized_value);
                            let now: i16 = Local::now().year().try_into().map_err(|_| error3)?;
                            let now = Year::new(now);
                            if now.to_number() - year.to_number() > 100 {
                                Err(serde::de::Error::invalid_value(unexpected, &EXPECTED))
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
