use serde::Deserialize;
use std::fmt::{self, Display};

/// Team distances
/// TODO: add support for laps and distance. This will generalize all team relays.
/// laps has to be a multiple of 2.
/// distance has to be an instance of `PoolLenght`.
#[derive(Deserialize, Debug, Clone, Copy)]
pub enum Team {
    /// 4 laps 25 meter per lap. Unnoficial distance
    #[serde(rename = "4*25")]
    Distance4x25,

    /// 4 laps 50 meter per lap
    #[serde(rename = "4*50")]
    Distance4x50,

    /// 6 laps 50 meter per lap
    #[serde(rename = "6*50")]
    Distance6x50,

    /// 4 laps 100 meter per lap
    #[serde(rename = "4*100")]
    Distance4x100,

    /// 8 laps 50 meter per lap
    #[serde(rename = "8*50")]
    Distance8x50,

    /// 4 laps 200 meter per lap
    #[serde(rename = "4*200")]
    Distance4x200,

    /// 4 laps 400 meter per lap
    #[serde(rename = "4*400")]
    Distance4x400,

    /// 1000m Unofficial distance.
    /// consider aliasing "5*4*50"
    #[serde(rename = "1000")]
    Distance1000,
}

impl TryFrom<isize> for Team {
    type Error = Error;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            100 => Ok(Self::Distance4x25),
            200 => Ok(Self::Distance4x50),
            300 => Ok(Self::Distance6x50),
            1000 => Ok(Self::Distance1000),
            400 | 800 => Err(Error::IndistinguishableDistance),
            _ => Err(Error::InvalidDistance),
        }
    }
}
impl TryFrom<&str> for Team {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "4*25" => Ok(Self::Distance4x25),
            "4*50" => Ok(Self::Distance4x50),
            "6*50" => Ok(Self::Distance6x50),
            "4*100" => Ok(Self::Distance4x100),
            "8*50" => Ok(Self::Distance8x50),
            "4*200" => Ok(Self::Distance4x200),
            "4*400" => Ok(Self::Distance4x400),
            "1000" => Ok(Self::Distance1000),
            _ => Err(Error::InvalidDistance),
        }
    }
}

#[allow(clippy::recursive_format_impl)]
impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match f.align() {
            None => match self {
                Self::Distance4x25 => write!(f, "4*25m"),
                Self::Distance4x50 => write!(f, "4*50m"),
                Self::Distance6x50 => write!(f, "6*50m"),
                Self::Distance4x100 => write!(f, "4*100m"),
                Self::Distance8x50 => write!(f, "8*50m"),
                Self::Distance4x200 => write!(f, "4*200m"),
                Self::Distance4x400 => write!(f, "4*400m"),
                Self::Distance1000 => write!(f, "1000m"),
            },
            Some(_) => f.pad(&self.to_string()),
        }
    }
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
    IndistinguishableDistance,
    Convert(std::num::ParseIntError),
    InvalidDistance,
}

#[allow(clippy::recursive_format_impl)]
impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match f.align() {
            Some(_) => f.pad(&self.to_string()),
            None => match self {
                Self::IndistinguishableDistance => {
                    write!(f, "cannot uniquly identify the distance")
                }
                Self::Convert(parse_error) => write!(f, "convertion error: {parse_error}"),
                Self::InvalidDistance => write!(f, "distance does not exists"),
            },
        }
    }
}
