extern crate core;
use self::core::fmt;
use serde::Deserialize;
use std::{convert::TryFrom, fmt::Display};

/// Individual distances
/// TODO: add support for adjustable distance. Distance has to be a multiple of `PoolLength`.
#[derive(Deserialize, Debug, Copy, Clone)]
pub enum Individual {
    #[serde(rename = "25")]
    Distance25 = 25,
    #[serde(rename = "50")]
    Distance50 = 50,
    #[serde(rename = "100")]
    Distance100 = 100,
    #[serde(rename = "150")]
    Distance150 = 150,
    #[serde(rename = "200")]
    Distance200 = 200,
    #[serde(rename = "400")]
    Distance400 = 400,
    #[serde(rename = "800")]
    Distance800 = 800,
    #[serde(rename = "1500")]
    Distance1500 = 1500,
}

impl TryFrom<isize> for Individual {
    type Error = Error;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            25 => Ok(Self::Distance25),
            50 => Ok(Self::Distance50),
            100 => Ok(Self::Distance100),
            150 => Ok(Self::Distance150),
            200 => Ok(Self::Distance200),
            400 => Ok(Self::Distance400),
            800 => Ok(Self::Distance800),
            1500 => Ok(Self::Distance1500),
            _ => Err(Error::InvalidDistance),
        }
    }
}

impl TryFrom<&str> for Individual {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed_number = value.parse::<isize>().map_err(Error::Convert)?;

        Self::try_from(parsed_number)
    }
}
#[allow(clippy::recursive_format_impl)]
impl fmt::Display for Individual {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match f.align() {
            None => write!(f, "{self}m"),
            Some(_) => f.pad(&self.to_string()),
        }
    }
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
    Convert(std::num::ParseIntError),
    InvalidDistance,
}

#[allow(clippy::recursive_format_impl)]
impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match f.align() {
            Some(_) => f.pad(&self.to_string()),
            None => match self {
                Self::Convert(parse_error) => write!(f, "conversion error: {parse_error}"),
                Self::InvalidDistance => write!(f, "distance does not exists"),
            },
        }
    }
}
