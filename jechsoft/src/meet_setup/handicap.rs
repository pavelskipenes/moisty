extern crate serde;
use self::serde::Deserialize;
use std::{convert::TryFrom, fmt::Display, num::ParseIntError};

#[derive(Debug, Clone, Copy)]
pub struct Handicap {
    /// Range of styles this dissability affects.
    pub style_group: StyleGroup,
    /// Disability type.
    ///
    /// 1 - 10 movement disability. Higher number means higher degree of dissability.
    /// 11 - 13 reduced eye sight up to full blindness. The higher the number the greated sight
    ///    loss.
    /// 14 mental dissability.
    /// 15 hearing loss
    pub disability_type: u8,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum StyleGroup {
    /// Freestyle, Backstroke and Butterfly
    FreestyleBackstrokeButterfly,
    /// Breast stroke
    BreastStroke,
    /// Medley
    Medley,
}

#[derive(Debug, thiserror::Error, Clone)]
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
    #[must_use]
    pub fn explain(self) -> String {
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
}

impl TryFrom<&str> for Handicap {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let style_group = match value {
            x if x.starts_with("SB") => StyleGroup::BreastStroke,
            x if x.starts_with("SM") => StyleGroup::Medley,
            x if x.starts_with('S') => StyleGroup::FreestyleBackstrokeButterfly,
            _ => Err(Error::InvalidHandicapStyleGroup)?,
        };
        let number: String = value
            .chars()
            .filter(|character| character.is_numeric())
            .collect();

        let disability_type = number.parse::<u8>()?;

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
        Self::try_from(deserialized_value.as_str()).map_err(|err| -> D::Error {
            serde::de::Error::invalid_value(serde::de::Unexpected::Str(&err.to_string()), &EXPECTED)
        })
    }
}
