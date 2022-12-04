use serde::Deserialize;
use std::fmt;

use super::error::Error;

#[derive(Debug, Clone)]
pub enum Distance {
    Individual(Individual),
    Team(Team),
}

/* impl Into<u8> for Distance {
    fn into(self) -> u8 {
        match self {
            Self::Individual(ind) => ind.into(),
            Self::Team(team) => team.into(),
        }
    }
}
 */
#[allow(clippy::recursive_format_impl)]
impl fmt::Display for Distance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match f.align() {
            // Need to split up self into team and individual. self.to_string() will recursively call this function,
            None => match self {
                Distance::Team(team) => write!(f, "{}", team),
                Distance::Individual(individual) => write!(f, "{}", individual),
            },
            Some(_) => f.pad(&self.to_string()),
        }
    }
}

impl Distance {
    pub fn is_official(&self) -> bool {
        match self {
            Distance::Team(team) => matches!(
                team,
                Team::Distance4x50 | Team::Distance4x100 | Team::Distance4x200
            ),

            Distance::Individual(individual) => matches!(
                individual,
                Individual::Distance50
                    | Individual::Distance100
                    | Individual::Distance200
                    | Individual::Distance400
                    | Individual::Distance800
                    | Individual::Distance1500
            ),
        }
    }

    fn from_string(s: &str) -> Result<Distance, Error> {
        match s {
            "25" => Ok(Distance::Individual(Individual::Distance25)),
            "50" => Ok(Distance::Individual(Individual::Distance50)),
            "100" => Ok(Distance::Individual(Individual::Distance100)),
            "150" => Ok(Distance::Individual(Individual::Distance150)),
            "200" => Ok(Distance::Individual(Individual::Distance200)),
            "400" => Ok(Distance::Individual(Individual::Distance400)),
            "800" => Ok(Distance::Individual(Individual::Distance800)),
            "1500" => Ok(Distance::Individual(Individual::Distance1500)),
            "4*25" => Ok(Distance::Team(Team::Distance4x25)),
            "4*50" => Ok(Distance::Team(Team::Distance4x50)),
            "6*50" => Ok(Distance::Team(Team::Distance6x50)),
            "8*50" => Ok(Distance::Team(Team::Distance8x50)),
            "4*100" => Ok(Distance::Team(Team::Distance4x100)),
            "4*200" => Ok(Distance::Team(Team::Distance4x200)),
            "4*400" => Ok(Distance::Team(Team::Distance4x400)),
            _ => Err(Error::InvalidDistance),
        }
    }
}

impl<'de> Deserialize<'de> for Distance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let deserialized_value: String = Deserialize::deserialize(deserializer)?;

        match Distance::from_string(&deserialized_value) {
            Ok(distance) => Ok(distance),
            Err(_err) => Err(serde::de::Error::unknown_variant(
                &deserialized_value,
                &[
                    "25", "50", "100", "200", "400", "800", "1500", "4*25", "4*50", "6*50",
                    "4*100", "4*200", "4*400",
                ],
            )),
        }
    }
}

/// Individual distances
#[derive(Deserialize, Debug, Clone)]
pub enum Individual {
    #[serde(rename = "25")]
    Distance25,
    #[serde(rename = "50")]
    Distance50,
    #[serde(rename = "100")]
    Distance100,
    #[serde(rename = "150")]
    Distance150,
    #[serde(rename = "200")]
    Distance200,
    #[serde(rename = "400")]
    Distance400,
    #[serde(rename = "800")]
    Distance800,
    #[serde(rename = "1500")]
    Distance1500,
}

// impl Into<u16> for Individual {
//     fn into(self) -> u16 {
//         match self {
//             Self::Distance25 => 25,
//             Self::Distance50 => 50,
//             Self::Distance100 => 100,
//             Self::Distance150 => 150,
//             Self::Distance200 => 200,
//             Self::Distance400 => 400,
//             Self::Distance800 => 800,
//             Self::Distance1500 => 1500,
//         }
//     }
// }

#[allow(clippy::recursive_format_impl)]
impl fmt::Display for Individual {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match f.align() {
            None => write!(f, "{}m", *self as u16),
            Some(_) => f.pad(&self.to_string()),
        }
    }
}

/// Team distances
#[derive(Deserialize, Debug, Clone)]
pub enum Team {
    /// 4 laps 25 meter per lap
    #[serde(rename = "4*25")]
    Distance4x25,

    /// 4 laps 50 meter per lap
    #[serde(rename = "4*50")]
    Distance4x50,

    /// 6 laps 50 meter per lap
    #[serde(rename = "6*50")]
    Distance6x50,

    /// 4 laps 100 meter per lap
    #[serde(rename = "4*100")]
    Distance4x100,

    /// 8 laps 50 meter per lap
    #[serde(rename = "8*50")]
    Distance8x50,

    /// 4 laps 200 meter per lap
    #[serde(rename = "4*200")]
    Distance4x200,

    /// 4 laps 400 meter per lap
    #[serde(rename = "4*400")]
    Distance4x400,
}
//#[allow(clippy::match_same_arms)]
//impl Into<u16> for Team {
//    fn into(self) -> u16 {
//        match self {
//            Self::Distance4x25 => 100,
//            Self::Distance4x50 => 200,
//            Self::Distance6x50 => 300,
//            Self::Distance4x100 => 400,
//            Self::Distance8x50 => 400,
//            Self::Distance4x200 => 800,
//            Self::Distance4x400 => 1600,
//        }
//    }
//}

#[allow(clippy::recursive_format_impl)]
impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match f.align() {
            None => match self {
                Team::Distance4x25 => write!(f, "4*25m"),
                Team::Distance4x50 => write!(f, "4*50m"),
                Team::Distance6x50 => write!(f, "6*50m"),
                Team::Distance4x100 => write!(f, "4*100m"),
                Team::Distance8x50 => write!(f, "8*50m"),
                Team::Distance4x200 => write!(f, "4*200m"),
                Team::Distance4x400 => write!(f, "4*400m"),
            },
            Some(_) => f.pad(&self.to_string()),
        }
    }
}
