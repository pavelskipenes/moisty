//! # Sorting
//!
//! Sorting is the process of distributing athletes across different heats within the same event.
//! Sorting algorithms sort athletes based on their class not on the athletes directly. Athletes
//! compete agains each other within their own class and the class it self is sorted against each
//! other. Multiple classec can be sorted within the same event but will never compete agains the
//! same prizes.
extern crate serde;
use self::serde::Deserialize;
use std::fmt::Display;

/// Sorting method
#[derive(Debug, Clone)]
pub enum Sorting {
    /// Sort with the fastest qualification time in the center of the pool and the last heat.
    /// Sort entries by enrollment time
    /// sortering order from shortest time to longst time: 3,4,2,5,1,6 for a 6 lane
    /// in an 8 lane: 4,3,5,2,6,1,7,8
    /// in an 10 lane: 5,4,6,3,7,2,8,1,9,0
    ///
    Final,
    /// Sorts the event by age and then time
    /// This is the default mode for unofficial meets
    FinalAgeGroupTime,
    /// Sorts the event by time and then by age. Honestly this could probably be more effective
    /// than `Sorting::FinalAgeGroupTime` if qualification times are present.
    FinalTimeAgeGroup,
    PartFinal,
    Alternative,
    FinalAgeGroupTimeSplitYF,
    Hcfinsrprejrfin,
    Preliminary,
    Hcpresrprejrfin,
    Hcfinsrfin,
    AgeGroupeDFinal,
}

#[allow(clippy::recursive_format_impl)]
impl Display for Sorting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match f.align() {
            Some(_) => f.pad(&self.to_string()),
            None => match self {
                Self::AgeGroupeDFinal => write!(f, "Age group D Final"),
                Self::Final => write!(f, "Final"),
                Self::FinalAgeGroupTime => write!(f, "Final, age then time"),
                Self::FinalTimeAgeGroup => write!(f, "Final, time then age"),
                Self::PartFinal => write!(f, "PartFinal"),
                Self::Alternative => write!(f, "Alternative"),
                Self::FinalAgeGroupTimeSplitYF => write!(f, "FinalAgeGroupTimeSplitYF"),
                Self::Hcfinsrprejrfin => write!(f, "HCfinsrprejrfin"),
                Self::Preliminary => write!(f, "preliminary"),
                Self::Hcpresrprejrfin => write!(f, "Hcpresrprejrfin"),
                Self::Hcfinsrfin => write!(f, "Hcfinsrfin"),
            },
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
            "AGEGROUPEDFINAL" => Ok(Self::AgeGroupeDFinal),
            "FINAL" => Ok(Self::Final),
            "PARTFINAL" => Ok(Self::PartFinal),
            "FINALAGEGROUPTIME" => Ok(Self::FinalAgeGroupTime),
            "FINALTIMEAGEGROUP" => Ok(Self::FinalTimeAgeGroup),
            "HCFINSRFIN" => Ok(Self::Hcfinsrfin),
            "PRELIMINARY" => Ok(Self::Preliminary),
            "ALTERNATIVE" => Ok(Self::Alternative),
            "HCFINSRPREJRFIN" => Ok(Self::Hcfinsrprejrfin),
            "HCPRESRPREJRFIN" => Ok(Self::Hcpresrprejrfin),
            "FINALAGEGROUPTIMESPLITYF" => Ok(Self::FinalAgeGroupTimeSplitYF),
            string => Err(serde::de::Error::custom(format!(
                "Could not decode {string} as Sorting type"
            ))),
        }
    }
}

// TODO: implement sorting methods below

// use num::Integer;
// pub const fn required_heats<T: Integer>(number_of_athletes: T, number_of_lanes: T) -> T {
//     number_of_athletes / number_of_lanes
// }
