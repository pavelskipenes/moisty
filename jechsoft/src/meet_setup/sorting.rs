use serde::Deserialize;

#[derive(Debug)]
pub enum Sorting {
    Alternative,
    /// Sort with the fastest qualification time in the center of the pool and the last heat.
    Final,
    FinalAgeGroupTime,
    FinalAgeGroupTimeSplitYF,
    FinalTimeAgeGroup,
    Hcfinsrprejrfin,
    Preliminary,
    Hcpresrprejrfin,
    Hcfinsrfin,
    PartFinal,
}

impl<'de> Deserialize<'de> for Sorting {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = serde::de::Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "FINAL" => Ok(Self::Final),
            "PARTFINAL" => Ok(Self::PartFinal),
            "HCFINSRFIN" => Ok(Self::Hcfinsrfin),
            "PRELIMINARY" => Ok(Self::Preliminary),
            "ALTERNATIVE" => Ok(Self::Alternative),
            "HCFINSRPREJRFIN" => Ok(Self::Hcfinsrprejrfin),
            "HCPRESRPREJRFIN" => Ok(Self::Hcpresrprejrfin),
            "FINALAGEGROUPTIME" => Ok(Self::FinalAgeGroupTime),
            "FINALTIMEAGEGROUP" => Ok(Self::FinalTimeAgeGroup),
            "FINALAGEGROUPTIMESPLITYF" => Ok(Self::FinalAgeGroupTimeSplitYF),
            string => Err(serde::de::Error::custom(format!(
                "Could not decode {string} as Sorting type"
            ))),
        }
    }
}
