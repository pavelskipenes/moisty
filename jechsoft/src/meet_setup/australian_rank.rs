extern crate serde;
use self::serde::Deserialize;
use std::{convert::TryFrom, fmt::Display};

#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum AustralianRank {
    Percent,
}

#[derive(Debug, thiserror::Error, Deserialize, Copy, Clone)]
pub enum Error {
    DoesNotExists,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DoesNotExists => write!(f, "australian rank does not exists"),
        }
    }
}

#[allow(clippy::recursive_format_impl)]
impl Display for AustralianRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match f.align() {
            Some(_) => f.pad(&self.to_string()),
            None => match self {
                Self::Percent => write!(f, "percent"),
            },
        }
    }
}

impl TryFrom<&str> for AustralianRank {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "percent" => Ok(Self::Percent),
            _ => Err(Error::DoesNotExists),
        }
    }
}
