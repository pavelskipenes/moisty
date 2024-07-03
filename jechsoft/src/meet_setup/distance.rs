use super::individual_distance::Error as IndividualError;
use super::team_distance::Error as TeamError;
use super::{individual_distance::Individual, team_distance::Team};
extern crate serde;
use self::serde::Deserialize;
use std::convert::TryFrom;
use std::fmt::{self, Display};

/// Distance in meters.
#[derive(Debug, Clone, Copy)]
pub enum Distance {
    Individual(Individual),
    Team(Team),
}

#[allow(clippy::recursive_format_impl)]
impl Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match f.align() {
            Some(_) => f.pad(&self.to_string()),
            None => match self {
                Self::Individual(individual) => f.write_str(&individual.to_string()),
                Self::Team(team) => f.write_str(&team.to_string()),
            },
        }
    }
}
impl TryFrom<&str> for Distance {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // try to return a team distance
        let team_err = match Team::try_from(value) {
            Ok(team) => return Ok(Self::Team(team)),
            Err(err) => err,
        };

        // try to return an individual distance
        let individual_err = match Individual::try_from(value) {
            Ok(individual) => return Ok(Self::Individual(individual)),
            Err(err) => err,
        };

        // could not construct distance so return the reason for failure
        Err(Error::TryFrom(individual_err, team_err))
    }
}

impl<'de> Deserialize<'de> for Distance {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let deserialized_value: String = Deserialize::deserialize(deserializer)?;
        let deserialize_error = serde::de::Error::unknown_variant(
            &deserialized_value,
            &[
                "25", "50", "100", "200", "400", "800", "1500", "4*25", "4*50", "6*50", "4*100",
                "4*200", "4*400",
            ],
        );
        // Note: This might be a fucked up way to do it because some distances might overlap and give false positives.
        Self::try_from(deserialized_value.as_str()).map_err(|_| deserialize_error)
    }
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
    TryFrom(IndividualError, TeamError),
}

#[allow(clippy::recursive_format_impl)]
impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match f.align() {
            Some(_) => f.pad(&self.to_string()),
            None => match self {
                Self::TryFrom(individual,team) => write!(f, "could not construct distance out of either individual nor team. Error individual: {individual}. Error team: {team}"),
            },
        }
    }
}
