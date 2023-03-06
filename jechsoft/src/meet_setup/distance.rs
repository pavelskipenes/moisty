use super::error::Error;
use serde::Deserialize;
use std::fmt;

/// Distance in meters.
#[derive(Debug, Clone)]
pub enum Distance {
    Individual(Individual),
    Team(Team),
}

#[allow(clippy::recursive_format_impl)]
impl fmt::Display for Distance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match f.align() {
            // Need to split up self into team and individual. self.to_string() will recursively call this function,
            None => match self {
                Self::Team(team) => write!(f, "{team}"),
                Self::Individual(individual) => {
                    write!(f, "{individual}")
                }
            },
            Some(_) => f.pad(&self.to_string()),
        }
    }
}

impl Distance {
    /// Returns true if current Distance is allowed in official meets
    #[must_use]
    pub fn is_official(&self) -> bool {
        match self {
            Self::Team(team) => matches!(
                team,
                Team::Distance4x50 | Team::Distance4x100 | Team::Distance4x200
            ),

            Self::Individual(individual) => matches!(
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
}

impl TryFrom<&str> for Distance {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "25" => Ok(Self::Individual(Individual::Distance25)),
            "50" => Ok(Self::Individual(Individual::Distance50)),
            "100" => Ok(Self::Individual(Individual::Distance100)),
            "150" => Ok(Self::Individual(Individual::Distance150)),
            "200" => Ok(Self::Individual(Individual::Distance200)),
            "400" => Ok(Self::Individual(Individual::Distance400)),
            "800" => Ok(Self::Individual(Individual::Distance800)),
            "1000" => Ok(Self::Team(Team::Distance1000)),
            "1500" => Ok(Self::Individual(Individual::Distance1500)),
            "4*25" => Ok(Self::Team(Team::Distance4x25)),
            "4*50" => Ok(Self::Team(Team::Distance4x50)),
            "6*50" => Ok(Self::Team(Team::Distance6x50)),
            "8*50" => Ok(Self::Team(Team::Distance8x50)),
            "4*100" => Ok(Self::Team(Team::Distance4x100)),
            "4*200" => Ok(Self::Team(Team::Distance4x200)),
            "4*400" => Ok(Self::Team(Team::Distance4x400)),
            _ => Err(Error::InvalidDistance),
        }
    }
}

impl TryFrom<&String> for Distance {
    type Error = Error;
    fn try_from(s: &String) -> Result<Self, Self::Error> {
        Self::try_from(s.as_str())
    }
}

impl<'de> Deserialize<'de> for Distance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let deserialized_value: String = Deserialize::deserialize(deserializer)?;

        match Self::try_from(&deserialized_value) {
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
    Distance25 = 25,
    #[serde(rename = "50")]
    Distance50 = 50,
    #[serde(rename = "100")]
    Distance100 = 100,
    #[serde(rename = "150")]
    Distance150 = 150,
    #[serde(rename = "200")]
    Distance200 = 200,
    #[serde(rename = "400")]
    Distance400 = 400,
    #[serde(rename = "800")]
    Distance800 = 800,
    #[serde(rename = "1500")]
    Distance1500 = 1500,
}

impl TryFrom<isize> for Individual {
    type Error = Error;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            25 => Ok(Self::Distance25),
            50 => Ok(Self::Distance50),
            100 => Ok(Self::Distance100),
            150 => Ok(Self::Distance150),
            200 => Ok(Self::Distance200),
            400 => Ok(Self::Distance400),
            800 => Ok(Self::Distance800),
            1500 => Ok(Self::Distance1500),
            _ => Err(Error::InvalidDistance),
        }
    }
}

#[allow(clippy::recursive_format_impl)]
impl fmt::Display for Individual {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match f.align() {
            None => write!(f, "{self}m"),
            Some(_) => f.pad(&self.to_string()),
        }
    }
}

/// Team distances
#[derive(Deserialize, Debug, Clone)]
pub enum Team {
    /// 4 laps 25 meter per lap. Unnoficial distance
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

    /// 1000m Unofficial distance.
    /// consider aliasing "5*4*50"
    #[serde(rename = "1000")]
    Distance1000,
}

impl TryFrom<isize> for Team {
    type Error = Error;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            100 => Ok(Self::Distance4x25),
            200 => Ok(Self::Distance4x50),
            300 => Ok(Self::Distance6x50),
            1000 => Ok(Self::Distance1000),
            400 | 800 => Err(Error::IndistinguishableDistance),
            _ => Err(Error::InvalidDistance),
        }
    }
}

#[allow(clippy::recursive_format_impl)]
impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match f.align() {
            None => match self {
                Self::Distance4x25 => write!(f, "4*25m"),
                Self::Distance4x50 => write!(f, "4*50m"),
                Self::Distance6x50 => write!(f, "6*50m"),
                Self::Distance4x100 => write!(f, "4*100m"),
                Self::Distance8x50 => write!(f, "8*50m"),
                Self::Distance4x200 => write!(f, "4*200m"),
                Self::Distance4x400 => write!(f, "4*400m"),
                Self::Distance1000 => write!(f, "1000m"),
            },
            Some(_) => f.pad(&self.to_string()),
        }
    }
}
