use std::num::ParseIntError;

use serde::Deserialize;

use super::{class::Class, gender_group::GenderGroup};

#[derive(Deserialize, Debug)]
pub struct GenderClass {
    pub gender_group: GenderGroup,
    pub class: Class,
}

pub enum Error {
    InvalidClassYearStr(ParseIntError),
    InvalidStrLen,
    InvalidGender,
}

impl GenderClass {
    pub fn from_str(input: &str) -> Result<Self, Error> {
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
                let gender_character = input.chars().next();
                let class_year_chars = &input[1..=2];
                let class_year = class_year_chars
                    .parse::<i64>()
                    .map_err(|parse_int_error| Error::InvalidClassYearStr(parse_int_error))?;
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
