use std::fmt;

use serde::Deserialize;

#[derive(Debug)]
pub enum Sorting {
    /// Undocumented.
    Alternative,
    /// Undocumented.
    Final,
    /// Undocumented.
    FinalAgeGroupTime,
    /// Undocumented.
    FinalAgeGroupTimeSplitYF,
    /// Undocumented.
    FinalTimeAgeGroup,
    /// Undocumented.
    Hcfinsrprejrfin,
    /// Undocumented.
    Preliminary,
    /// Undocumented.
    Hcpresrprejrfin,
    /// Undocumented.
    Hcfinsrfin,
    /// Undocumented.
    PartFinal,
}

#[allow(clippy::recursive_format_impl)]
impl fmt::Display for Sorting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match f.align() {
            None => write!(
                f,
                "{}",
                match self {
                    Sorting::Final => "[undocumented] final",
                    Sorting::Alternative => "[undocumented] alternative",
                    Sorting::FinalAgeGroupTime => "[undocumented] final-age-group-time",
                    Sorting::FinalTimeAgeGroup => "[undocumented] final-time-age-group",
                    Sorting::FinalAgeGroupTimeSplitYF =>
                        "[undocumented] final-age-group-time-split-yf",
                    Sorting::Hcfinsrprejrfin => "[undocumented] hc-fin-sr-pre-jr-fin",
                    Sorting::Hcfinsrfin => "[undocumented] hc-fin-sr-fin",
                    Sorting::Preliminary => "[undocumented] preliminary",
                    Sorting::Hcpresrprejrfin => "[undocumented] hc-pre-sr-pre-jr-fin",
                    Sorting::PartFinal => "[undocumented] part-final",
                }
            ),
            Some(_) => f.pad(&self.to_string()),
        }
    }
}

impl<'de> Deserialize<'de> for Sorting {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = serde::de::Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "FINAL" => Ok(Sorting::Final),
            "PARTFINAL" => Ok(Sorting::PartFinal),
            "HCFINSRFIN" => Ok(Sorting::Hcfinsrfin),
            "PRELIMINARY" => Ok(Sorting::Preliminary),
            "ALTERNATIVE" => Ok(Sorting::Alternative),
            "HCFINSRPREJRFIN" => Ok(Sorting::Hcfinsrprejrfin),
            "HCPRESRPREJRFIN" => Ok(Sorting::Hcpresrprejrfin),
            "FINALAGEGROUPTIME" => Ok(Sorting::FinalAgeGroupTime),
            "FINALTIMEAGEGROUP" => Ok(Sorting::FinalTimeAgeGroup),
            "FINALAGEGROUPTIMESPLITYF" => Ok(Sorting::FinalAgeGroupTimeSplitYF),
            string => Err(serde::de::Error::custom(format!(
                "Could not decode {} as Sorting type",
                string
            ))),
        }
    }
}
