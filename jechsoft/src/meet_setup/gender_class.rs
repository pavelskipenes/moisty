use super::{class::Class, gender_group::GenderGroup};
use serde::Deserialize;
use std::num::ParseIntError;

#[derive(Deserialize, Debug)]
pub struct GenderClass {
    pub gender_group: GenderGroup,
    pub class: Class,
}

pub enum Error {
    InvalidClassYearStr(ParseIntError),
    InvalidStrLen,
    InvalidGender,
    Todo,
}
impl std::str::FromStr for GenderClass {
    type Err = crate::meet_setup::gender_class::Error;

    /// # Errors
    /// - returns `Error::InvalidStrLen` if input is not 3 characters long
    /// - returns `Error::InvalidClassYearStr`
    /// - returns `Error::InvalidGender`
    fn from_str(input: &str) -> Result<Self, Self::Err> {
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
                        .parse::<i64>()
                        .map_err(Error::InvalidClassYearStr)?;

                    let gender_character = input.chars().next();
                    match gender_character {
                        Some('M') => Ok(Self {
                            gender_group: GenderGroup::Male,
                            class: Class::Junior(Some(datetime::Year(class_year))),
                        }),
                        Some('K') => Ok(Self {
                            gender_group: GenderGroup::Female,
                            class: Class::Junior(Some(datetime::Year(class_year))),
                        }),
                        _ => Err(Error::InvalidGender),
                    }
                }
            }
        }
    }
}
