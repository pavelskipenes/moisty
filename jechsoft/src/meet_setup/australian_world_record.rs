extern crate serde;
use self::serde::Deserialize;
use std::{convert::TryFrom, fmt::Display};

#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum AustralianWorldRecord {
    LongCourse,
    Same,
}

#[derive(thiserror::Error, Debug, Copy, Clone, Deserialize)]
pub enum Error {
    DoesNotExists,
}

#[allow(clippy::recursive_format_impl)]
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match f.align() {
            Some(_) => f.pad(&self.to_string()),
            None => match self {
                Self::DoesNotExists => write!(f, "variant does not exists"),
            },
        }
    }
}

impl TryFrom<&str> for AustralianWorldRecord {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "long course" => Ok(Self::LongCourse),
            "same" => Ok(Self::Same),
            _ => Err(Error::DoesNotExists),
        }
    }
}
