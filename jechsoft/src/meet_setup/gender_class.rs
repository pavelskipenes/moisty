extern crate chrono;
extern crate gregorian;
extern crate serde;
use self::gregorian::Year;
use self::serde::Deserialize;
use super::{class::Class, gender_group::GenderGroup};
use std::{convert::TryFrom, fmt::Display, num::ParseIntError};

#[derive(Deserialize, Debug)]
pub struct GenderClass {
    pub gender_group: GenderGroup,
    pub class: Class,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    InvalidClassYearStr(ParseIntError),
    InvalidStrLen,
    InvalidGender,
}

#[allow(clippy::recursive_format_impl)]
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match f.align() {
            Some(_) => f.pad(&self.to_string()),
            None => match self {
                Self::InvalidClassYearStr(err) => {
                    write!(f, "invalid class year: {err}")
                }
                Self::InvalidStrLen => write!(f, "invalid string length. Expected 3 characters"),
                Self::InvalidGender => write!(f, "invalid gender character. Expected 'M' | 'K'."),
            },
        }
    }
}

impl TryFrom<&str> for GenderClass {
    type Error = Error;

    /// # Errors
    /// - returns `Error::InvalidStrLen` if input is not 3 characters long
    /// - returns `Error::InvalidClassYearStr`
    /// - returns `Error::InvalidGender`
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        {
            if input.len() != 3 {
                return Err(Error::InvalidStrLen);
            }
            match input {
                "MSR" => Ok(Self {
                    gender_group: GenderGroup::Male,
                    class: Class::Senior,
                }),
                "MJS" => Ok(Self {
                    gender_group: GenderGroup::Male,
                    class: Class::Junior(None),
                }),
                "KSR" => Ok(Self {
                    gender_group: GenderGroup::Female,
                    class: Class::Senior,
                }),
                "KJR" => Ok(Self {
                    gender_group: GenderGroup::Female,
                    class: Class::Junior(None),
                }),
                "XSR" => Ok(Self {
                    gender_group: GenderGroup::Mixed,
                    class: Class::Senior,
                }),
                "XJR" => Ok(Self {
                    gender_group: GenderGroup::Mixed,
                    class: Class::Junior(None),
                }),
                input => {
                    // the only valid input is one letter gender character and two letter birth year

                    let class_year: String = input.chars().take(2).collect();
                    let class_year = class_year
                        .parse::<i16>()
                        .map_err(Error::InvalidClassYearStr)?;

                    let gender_character = input.chars().next();
                    match gender_character {
                        Some('M') => Ok(Self {
                            gender_group: GenderGroup::Male,
                            class: Class::Junior(Some(Year::new(class_year))),
                        }),
                        Some('K') => Ok(Self {
                            gender_group: GenderGroup::Female,
                            class: Class::Junior(Some(Year::new(class_year))),
                        }),
                        _ => Err(Error::InvalidGender),
                    }
                }
            }
        }
    }
}
