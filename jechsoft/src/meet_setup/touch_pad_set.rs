use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Clone, Copy)]
pub enum TouchPadSet {
    OneSet,
    TwoSet,
    None,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    UnknownSet,
}

#[allow(clippy::recursive_format_impl)]
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match f.align() {
            Some(_) => f.pad(&self.to_string()),
            None => match self {
                Self::UnknownSet => write!(f, "unknown touch pad set"),
            },
        }
    }
}
impl TryFrom<&str> for TouchPadSet {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ONE SET" => Ok(Self::OneSet),
            "TWO SET" => Ok(Self::TwoSet),
            "NO" => Ok(Self::None),
            _ => Err(Error::UnknownSet),
        }
    }
}

impl<'de> Deserialize<'de> for TouchPadSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = serde::de::Deserialize::deserialize(deserializer)?;
        Self::try_from(s.as_str()).map_or_else(
            |_| {
                Err(serde::de::Error::custom(format!(
                    "Could not decode '{s}' as TouchPadSet type"
                )))
            },
            Ok,
        )
    }
}
