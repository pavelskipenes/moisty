use serde::Deserialize;

#[derive(Debug)]
pub enum TouchPadSet {
    OneSet,
}

impl<'de> Deserialize<'de> for TouchPadSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = serde::de::Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "ONE SET" => Ok(Self::OneSet),
            string => Err(serde::de::Error::custom(format!(
                "Could not decode {} as TouchPadSet type",
                string
            ))),
        }
    }
}
