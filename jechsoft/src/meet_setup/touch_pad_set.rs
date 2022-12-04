use std::fmt;

use serde::Deserialize;

#[derive(Debug)]
pub enum TouchPadSet {
    OneSet,
}

#[allow(clippy::recursive_format_impl)]
impl fmt::Display for TouchPadSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match f.align() {
            None => write!(
                f,
                "{}",
                match self {
                    TouchPadSet::OneSet => "one set",
                }
            ),
            Some(_) => f.pad(&self.to_string()),
        }
    }
}

impl<'de> Deserialize<'de> for TouchPadSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = serde::de::Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "ONE SET" => Ok(TouchPadSet::OneSet),
            string => Err(serde::de::Error::custom(format!(
                "Could not decode {} as TouchPadSet type",
                string
            ))),
        }
    }
}
