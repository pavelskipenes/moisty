/// `uni_p.txt` is a csv filetype that contains enrollment information
use crate::meet_setup::{
    distance::Distance, gender_class::GenderClass, gender_group::GenderGroup,
    pool_length::PoolLength, style::Style,
};
extern crate serde;
use self::serde::Deserialize;
use std::{fmt::Display, num::ParseIntError, time::Duration};

#[derive(Debug, Deserialize)]
pub struct EnrollmentEntry {
    pub event_number: u8,
    pub distance: Distance,
    pub style: Style,
    pub name: String,
    pub enrollment_time: Duration,
    pub pool_length: PoolLength,
    pub gender_group: GenderGroup,
    pub gender_class: GenderClass,
    pub enrollment_variant: EnrollmentVariant,
}

#[derive(Debug, Deserialize, Clone)]
pub enum EnrollmentVariant {
    Individual(EnrollmentIndividual),
    Relay(EnrollmentRelay),
}

#[derive(Debug, Deserialize, Clone)]
pub struct EnrollmentIndividual {
    pub name: String,
    pub surname: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EnrollmentRelay {
    pub team_name: String,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum Field {
    EventNumber,
    Distance,
    Duration,
    PoolLength,
}

#[allow(clippy::recursive_format_impl)]
impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match f.align() {
            Some(_) => f.pad(&self.to_string()),
            None => match self {
                Self::EventNumber => write!(f, "event number"),
                Self::Distance => write!(f, "distance"),
                Self::Duration => write!(f, "duration"),
                Self::PoolLength => write!(f, "pool length"),
            },
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    // TODO: add line number in the error output
    // TODO: add &str of what input we were trying to parse
    // TODO: add [&str] of whatever we expected
    Missing(Field),
    Unrecognized(Field),
    Parse(Field, ParseIntError),
}

#[allow(clippy::recursive_format_impl)]
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match f.align() {
            Some(_) => f.pad(&self.to_string()),
            None => match self {
                Self::Missing(field) => write!(f, "missing field: {field}"),
                Self::Unrecognized(field) => write!(f, "unrecognized field: {field}"),
                Self::Parse(field, err) => write!(f, "parse error on field: {field}. {err}"),
            },
        }
    }
}
// fn str_to_duration(input: &str) -> Result<Duration, Error> {
//     let mut split = input.split(&[':', '.'][..]);
//     let decomposed_duration = split
//         .next_chunk::<3>()
//         .map_err(|_| Error::Unknown)?
//         .map(|val| val.parse::<u64>().ok());
//
//     match decomposed_duration {
//         [Some(minutes), Some(seconds), Some(deciseconds)] => Ok(Duration::from_millis(
//             deciseconds * 10 + seconds * 1000 + minutes * 60 * 1000,
//         )),
//         _ => Err(Error::Unknown),
//     }
// }

// /// `uni_p.txt` deserializer
// pub fn deserialize_csv(input: &str) -> Result<(String, Vec<EnrollmentEntry>), Error> {
//     let mut entries = Vec::new();
//     let club_name: String = input.lines().take(1).collect();
//
//     for (line_number, line) in input.lines().enumerate().skip(1) {
//         let mut fields = line.split(',');
//
//         entries.push(EnrollmentEntry {
//             event_number: fields
//                 .next()
//                 .ok_or(Error::Missing(Field::EventNumber))?
//                 .parse::<u8>()
//                 .map_err(|parse_int_error| Error::Parse(Field::EventNumber, parse_int_error))?,
//
//             distance: Distance::try_from(
//                 fields
//                     .next()
//                     .ok_or(Error::Missing)
//                     .map_err(|_| Error::Unknown)?,
//             )
//             .map_err(|_| Error::Unknown)?,
//
//             style: Style::try_from(
//                 fields
//                     .next()
//                     .ok_or(Error::Missing)
//                     .map_err(|_| Error::Unknown)?,
//             )
//             .map_err(|_| Error::Unknown)?,
//             name: format!("{} {}", fields.next().unwrap(), fields.next().unwrap()),
//             // skip 1
//             gender_group: write!(f, ""),
//             gender_class: write!(f, ""),
//             // class
//             // year
//             enrollment_time: str_to_duration(fields.next().unwrap()).map_err(|_| Error::Unknown)?,
//
//             pool_length: match fields.next() {
//                 Some("K") => PoolLength::PoolLength25,
//                 Some("L") => PoolLength::PoolLength50,
//                 _ => Err(Error::Unrecognized(Field::PoolLength))?,
//             },
//             enrollment_variant: write!(f, ""),
//         });
//     }
//
//     Ok((club_name, entries))
// }
