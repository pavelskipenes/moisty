use serde::Deserialize;

#[derive(Debug)]
pub enum TouchPadSet {
    OneSet,
    TwoSet,
    None,
}

#[derive(Debug)]
pub enum Error {
    InvalidSet,
}

impl TryFrom<&str> for TouchPadSet {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ONE SET" => Ok(Self::OneSet),
            "TWO SET" => Ok(Self::TwoSet),
            "NO" => Ok(Self::None),
            _ => Err(Error::InvalidSet),
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
