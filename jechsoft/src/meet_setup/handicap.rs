use serde::Deserialize;
use std::{fmt::Display, num::ParseIntError};

#[derive(Debug)]
pub struct Handicap {
    /// Undocumented field.
    pub style_group: StyleGroup,
    /// Undocumented field.
    pub disability_type: u8,
}

#[derive(Debug)]
pub enum StyleGroup {
    FreestyleBackstrokeButterfly,
    BreastStroke,
    Medley,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    ParseIntError(#[from] ParseIntError),
    InvalidHandicapStyleGroup,
}

#[allow(clippy::recursive_format_impl)]
impl Display for Handicap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.align().is_some() {
            f.pad(&self.to_string())
        } else {
            let result = match self.style_group {
                StyleGroup::FreestyleBackstrokeButterfly => "S",
                StyleGroup::BreastStroke => "SB",
                StyleGroup::Medley => "SM",
            };

            write!(f, "{},{}", result, self.disability_type)
        }
    }
}

#[allow(clippy::recursive_format_impl)]
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match f.align() {
            Some(_) => f.pad(&self.to_string()),
            None => match self {
                Self::ParseIntError(error) => write!(f, "{error}"),
                Self::InvalidHandicapStyleGroup => write!(f, "invalid style group"),
            },
        }
    }
}

impl Handicap {
    pub fn explain(&self) -> String {
        let disability_type = match self.disability_type {
            1..=10 => "movement and mobility",
            11..=13 => "reduced eye sight or blind",
            14 => "mental disability",
            15 => "deaf",
            0 | 16..=u8::MAX => panic!("invalid disability grade"),
        };
        let styles = match self.style_group {
            StyleGroup::FreestyleBackstrokeButterfly => "freestyle, backstroke and butterfly",
            StyleGroup::BreastStroke => "breaststroke",
            StyleGroup::Medley => "medley",
        };
        format!("{disability_type} in {styles}")
    }

    pub fn from_str(str_input: &str) -> Result<Self, Error> {
        let mut starting_index = 2;
        let style_group = match str_input[0..2].to_ascii_uppercase().as_str() {
            "SB" => StyleGroup::BreastStroke,
            "SM" => StyleGroup::Medley,
            two_char_thingy => {
                if !two_char_thingy.starts_with('S') {
                    Err(Error::InvalidHandicapStyleGroup)?;
                }
                starting_index = 1;
                StyleGroup::FreestyleBackstrokeButterfly
            }
        };
        let disability_type = str_input[starting_index..].parse::<u8>()?;

        Ok(Self {
            style_group,
            disability_type,
        })
    }
}

impl<'de> Deserialize<'de> for Handicap {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const EXPECTED: &str = "Sx, SMx, SBx where x is a number between 1 and 15";

        let deserialized_value: String = Deserialize::deserialize(deserializer)?;
        Self::from_str(deserialized_value.as_str()).map_err(|err| -> D::Error {
            serde::de::Error::invalid_value(serde::de::Unexpected::Str(&err.to_string()), &EXPECTED)
        })
    }
}
