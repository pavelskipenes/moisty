/// `uni_p.txt` is a csv filetype that contains enrollment information
use crate::meet_setup::{
    distance::Distance, gender_class::GenderClass, gender_group::GenderGroup,
    pool_length::PoolLength, style::Style,
};
use serde::Deserialize;
use std::{num::ParseIntError, time::Duration};

#[derive(Deserialize)]
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

#[derive(Debug, Deserialize)]
pub enum EnrollmentVariant {
    Individual(EnrollmentIndividual),
    Relay(EnrollmentRelay),
}

#[derive(Debug, Deserialize)]
pub struct EnrollmentIndividual {
    pub name: String,
    pub surname: String,
}

#[derive(Debug, Deserialize)]
pub struct EnrollmentRelay {
    pub team_name: String,
}

pub enum Field {
    EventNumber,
    Distance,
    Duration,
    PoolLength,
}

pub enum Error {
    // TODO: add line number in the error output
    // TODO: add &str of what input we were trying to parse
    // TODO: add [&str] of whatever we expected
    Missing(Field),
    Unrecognized(Field),
    Parse(Field, ParseIntError),
    // I'm too lazy to fix error mappings just yet
    Unknown,
}

fn str_to_duration(input: &str) -> Result<Duration, Error> {
    let mut split = input.split(&[':', '.'][..]);
    let decomposed_duration = split
        .next_chunk::<3>()
        .map_err(|_| Error::Unknown)?
        .map(|val| val.parse::<u64>().ok());

    match decomposed_duration {
        [Some(minutes), Some(seconds), Some(deciseconds)] => Ok(Duration::from_millis(
            deciseconds * 10 + seconds * 1000 + minutes * 60 * 1000,
        )),
        _ => Err(Error::Unknown),
    }
}

pub fn deserialize_csv(input: &str) -> Result<(String, Vec<EnrollmentEntry>), Error> {
    let mut entries = Vec::new();
    let club_name: String = input.lines().take(1).collect();

    for (line_number, line) in input.lines().enumerate().skip(1) {
        let mut fields = line.split(',');

        entries.push(EnrollmentEntry {
            event_number: fields
                .next()
                .ok_or(Error::Missing(Field::EventNumber))?
                .parse::<u8>()
                .map_err(|parse_int_error| Error::Parse(Field::EventNumber, parse_int_error))?,

            distance: Distance::try_from(
                fields
                    .next()
                    .ok_or(Error::Missing)
                    .map_err(|_| Error::Unknown)?,
            )
            .map_err(|_| Error::Unknown)?,

            style: Style::try_from(
                fields
                    .next()
                    .ok_or(Error::Missing)
                    .map_err(|_| Error::Unknown)?,
            )
            .map_err(|_| Error::Unknown)?,
            name: format!("{} {}", fields.next().unwrap(), fields.next().unwrap()),
            // skip 1
            gender_group: todo!(),
            gender_class: todo!(),
            // class
            // year
            enrollment_time: str_to_duration(fields.next().unwrap()).map_err(|_| Error::Unknown)?,

            pool_length: match fields.next() {
                Some("K") => PoolLength::PoolLength25,
                Some("L") => PoolLength::PoolLength50,
                _ => Err(Error::Unrecognized(Field::PoolLength))?,
            },
            enrollment_variant: todo!(),
        });
    }

    Ok((club_name, entries))
}
