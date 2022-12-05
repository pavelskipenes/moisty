use serde::Deserialize;
use std::fmt;

#[derive(Debug)]
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

        match deserialized_value.as_ref() {
            "3" => Ok(Self::Third),
            "NO" => Ok(Self::None),
            "MEDALS" => Ok(Self::Medals),
            "DEFAULT" => Ok(Self::Default),
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
                Self::Default => write!(f, "events default"),
                Self::Medals => write!(f, "medals"),
                Self::None => write!(f, "none"),
                Self::Third => write!(f, "top 1/3"),
            },
            Some(_) => f.pad(&self.to_string()),
        }
    }
}
