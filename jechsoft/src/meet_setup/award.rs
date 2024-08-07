extern crate serde;

use self::serde::Deserialize;
use std::{
    convert::TryFrom,
    fmt::{self, Display},
};

#[derive(Debug, Copy, Clone)]
pub enum Award {
    Default,
    Medals,
    None,
    Third,
}

impl<'de> Deserialize<'de> for Award {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let deserialized_value: String = Deserialize::deserialize(deserializer)?;
        let unknown_variant_error: D::Error = serde::de::Error::unknown_variant(
            &deserialized_value,
            &["DEFAULT", "MEDALS", "NO", "3"],
        );
        Self::try_from(deserialized_value.as_ref()).map_err(|_| unknown_variant_error)
    }
}

#[derive(Debug, thiserror::Error, Clone, Copy, Deserialize)]
pub enum Error {
    UnknownVariant,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownVariant => write!(f, "unknown variant"),
        }
    }
}

impl TryFrom<&str> for Award {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "3" => Ok(Self::Third),
            "NO" => Ok(Self::None),
            "MEDALS" => Ok(Self::Medals),
            "DEFAULT" => Ok(Self::Default),
            _ => Err(Error::UnknownVariant),
        }
    }
}

#[allow(clippy::recursive_format_impl)]
impl fmt::Display for Award {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match f.align() {
            None => match self {
                Self::Default => write!(f, "events default"),
                Self::Medals => write!(f, "medals"),
                Self::None => write!(f, "none"),
                Self::Third => write!(f, "top 1/3"),
            },
            Some(_) => f.pad(&self.to_string()),
        }
    }
}
