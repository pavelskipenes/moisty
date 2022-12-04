use serde::Deserialize;
use std::fmt;

#[derive(Debug)]
pub enum Award {
    /// Unconfirmed: use event defaults
    Default,
    /// Unconfirmed: medals to all participants
    Medals,
    /// Unconfirmed: No awards
    None,
    /// Unconfirmed: awards to top 1/3 of the results
    Third,
}

impl<'de> Deserialize<'de> for Award {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let deserialized_value: String = Deserialize::deserialize(deserializer)?;

        match deserialized_value.as_ref() {
            "3" => Ok(Award::Third),
            "NO" => Ok(Award::None),
            "MEDALS" => Ok(Award::Medals),
            "DEFAULT" => Ok(Award::Default),
            _ => Err(serde::de::Error::unknown_variant(
                &deserialized_value,
                &["DEFAULT", "MEDALS", "NO", "3"],
            )),
        }
    }
}

#[allow(clippy::recursive_format_impl)]
impl fmt::Display for Award {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match f.align() {
            None => match self {
                Award::Default => write!(f, "events default"),
                Award::Medals => write!(f, "medals"),
                Award::None => write!(f, "none"),
                Award::Third => write!(f, "top 1/3"),
            },
            Some(_) => f.pad(&self.to_string()),
        }
    }
}
