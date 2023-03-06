use serde::Deserialize;

#[derive(Debug)]
pub enum TouchPadSet {
    OneSet,
    TwoSet,
}

impl<'de> Deserialize<'de> for TouchPadSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = serde::de::Deserialize::deserialize(deserializer)?;
        // bug: for some reason "TWO SET" is not matching the second arm here.
        match s.as_str() {
            "ONE SET" => Ok(Self::OneSet),
            "TWO SET" => Ok(Self::TwoSet),
            string => {
                dbg!(&string);
                Err(serde::de::Error::custom(format!(
                    "Could not decode '{string}' as TouchPadSet type"
                )))
            }
        }
    }
}
