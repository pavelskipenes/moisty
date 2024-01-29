use serde::Deserialize;
use std::{
    convert::{TryFrom, TryInto},
    fmt::{self, Display},
};

/// Team distances
/// constructed from number of laps as the first argument and distance in the second argument.
/// Total distance is number of laps multiplied by distance per lap
#[derive(Debug, Clone, Copy)]
pub enum Team {
    /// Distance(laps,Distance)
    Distance(u8, u16),
}

impl<'de> Deserialize<'de> for Team {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let expected = "num_laps*total_distance. two numbers separated by a '*' character. example: '2*50', '20*50', '4'100'";
        let value: String = serde::de::Deserialize::deserialize(deserializer)?;
        let unexpected = serde::de::Unexpected::Str(value.as_ref());
        let parse_error: D::Error = serde::de::Error::invalid_value(unexpected, &expected);

        // extract two numbers separated by '*' character
        let parsed_numbers = value
            .split('*')
            .filter_map(|maybe_number| maybe_number.parse::<u16>().ok())
            .array_chunks::<2>()
            .next();

        match parsed_numbers {
            Some([num_laps, total_distance]) if num_laps % 2 == 0 && total_distance % 25 == 0 => {
                Ok(Self::Distance(
                    num_laps.try_into().map_err(|_| parse_error)?,
                    total_distance,
                ))
            },
            Some([_,_]) => Err(serde::de::Error::invalid_value(unexpected, &"number of laps has to be a multiple of 2 and distance per lap has to be a multiple of 25")),
            _ => Err(parse_error),
        }
    }
}

impl TryFrom<isize> for Team {
    type Error = Error;

    fn try_from(_value: isize) -> Result<Self, Self::Error> {
        Err(Error::InvalidDistance)
    }
}
impl TryFrom<&str> for Team {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parsed_numbers = value
            .split('*')
            .filter_map(|maybe_number| maybe_number.parse::<u16>().ok())
            .array_chunks::<2>()
            .next();

        match parsed_numbers {
            Some([num_laps, total_distance]) if num_laps % 2 == 0 && total_distance % 25 == 0 => {
                Ok(Self::Distance(
                    num_laps.try_into().unwrap(),
                    // .try_into()
                    // .map_err(|convert_error| Error::Convert(convert_error))?,
                    total_distance,
                ))
            }
            Some([_, _]) | None => Err(Error::InvalidDistance),
        }
    }
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
    IndistinguishableDistance,
    Convert(std::num::ParseIntError),
    InvalidDistance,
}

// #[allow(clippy::recursive_format_impl)]
// impl fmt::Display for Team {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match f.align() {
//             None => match self {
//             },
//             Some(_) => f.pad(&self.to_string()),
//         }
//     }
// }
#[allow(clippy::recursive_format_impl)]
impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match f.align() {
            Some(_) => f.pad(&self.to_string()),
            None => match self {
                Self::IndistinguishableDistance => {
                    write!(f, "cannot uniquely identify the distance")
                }
                Self::Convert(parse_error) => write!(f, "conversion error: {parse_error}"),
                Self::InvalidDistance => write!(f, "distance does not exists"),
            },
        }
    }
}
