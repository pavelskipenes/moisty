use serde::Deserialize;
use std::{fmt::Display, convert::TryFrom};

/// Length of the pool
#[derive(Deserialize, Debug, Clone, Copy)]
pub enum PoolLength {
    /// 25 meters pool. Often called "short course".
    #[serde(rename = "25")]
    PoolLength25,
    /// 50 meters pool. Often called "long course".
    #[serde(rename = "50")]
    PoolLength50,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    PoolLengthDoesNotExists,
    TryFrom(std::num::ParseIntError),
}

impl TryFrom<u8> for PoolLength {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            25 => Ok(Self::PoolLength25),
            50 => Ok(Self::PoolLength50),
            _ => Err(Error::PoolLengthDoesNotExists),
        }
    }
}

impl TryFrom<&str> for PoolLength {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.parse::<u8>() {
            Ok(value) => Self::try_from(value),
            Err(err) => Err(Error::TryFrom(err)),
        }
    }
}

#[allow(clippy::recursive_format_impl)]
impl Display for PoolLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match f.align() {
            Some(_) => f.pad(&self.to_string()),
            None => match self {
                Self::PoolLength50 => write!(f, "50m"),
                Self::PoolLength25 => write!(f, "25m"),
            },
        }
    }
}

#[allow(clippy::recursive_format_impl)]
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match f.align() {
            Some(_) => f.pad(&self.to_string()),
            None => match self {
                Self::PoolLengthDoesNotExists => write!(f, "pool length does not exists"),
                Self::TryFrom(parse_err) => write!(f, "TryFrom: {parse_err}"),
            },
        }
    }
}
