use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Round {
    /// Undocumented variant.
    Final,
    /// Undocumented variant.
    Final8,
    /// Undocumented variant.
    DirectFinal,
    /// Undocumented variant.
    QuarterFinal,
    /// Undocumented variant.
    SemiFinal,
    /// Undocumented variant.
    Preliminary,
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Round>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: String = serde::de::Deserialize::deserialize(deserializer)?;
    match s.as_str() {
        "8FINAL" => Ok(Some(Round::Final8)),
        "DIRECTFINAL" => Ok(Some(Round::DirectFinal)),
        "SEMIFINAL" => Ok(Some(Round::SemiFinal)),
        "FINAL" => Ok(Some(Round::Final)),
        "PRELIMINARY" => Ok(Some(Round::Preliminary)),
        "UNDEFINED" => Ok(None),
        string => Err(serde::de::Error::custom(format!(
            "Could not decode {} as Round type",
            string
        ))),
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
                    Round::Final => "final",
                    Round::SemiFinal => "semi final",
                    Round::QuarterFinal => "quarter final",
                    Round::Final8 => "8 final",
                    Round::DirectFinal => "direct final",
                    Round::Preliminary => "preliminary",
                }
            ),
            Some(_) => f.pad(&self.to_string()),
        }
    }
}
