extern crate serde;
use self::serde::Deserialize;
use std::fmt;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Round {
    Final,
    Final8,
    DirectFinal,
    QuarterFinal,
    SemiFinal,
    Preliminary,
    Undefined,
}

impl<'de> Deserialize<'de> for Round {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = serde::de::Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "8FINAL" => Ok(Self::Final8),
            "DIRECTFINAL" => Ok(Self::DirectFinal),
            "SEMIFINAL" => Ok(Self::SemiFinal),
            "FINAL" => Ok(Self::Final),
            "PRELIMINARY" => Ok(Self::Preliminary),
            "QUARTERFINAL" => Ok(Self::QuarterFinal),
            "UNDEFINED" => Ok(Self::Undefined),
            string => Err(serde::de::Error::custom(format!(
                "Could not decode {string} as Round type"
            ))),
        }
    }
}

#[allow(clippy::recursive_format_impl)]
impl fmt::Display for Round {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match f.align() {
            None => write!(
                f,
                "{}",
                match self {
                    Self::Final => "final",
                    Self::SemiFinal => "semi final",
                    Self::QuarterFinal => "quarter final",
                    Self::Final8 => "8 final",
                    Self::DirectFinal => "direct final",
                    Self::Preliminary => "preliminary",
                    Self::Undefined => "undefined",
                }
            ),
            Some(_) => f.pad(&self.to_string()),
        }
    }
}
