use super::error::Error;

/// `Athlete`s between 9 and 19 years old gets placed based on their age.
/// `Athlete`s class is based on their age at the end of the year, the same year as the
/// meet date. In other words, athletes that are born the same year are competing in the
/// same class regardless of the date of the meet.
#[derive(Debug)]
pub enum Junior {
    /// 9 years old
    A = 9,

    /// 10 years old
    B = 10,

    /// 11 years old
    C = 11,

    /// 12 years old
    D = 12,

    /// 13 years old
    E = 13,

    /// 14 years old
    F = 14,

    /// 15 years old
    G = 15,

    /// 16 years old
    H = 16,

    /// 17 years old
    I = 17,

    /// 18 years old
    J = 18,

    /// 19 years old
    K = 19,
}

// cannot implement tryFrom<Year> because class is dependent on the year when the meet
// is taking place. Assuming inside a function that it's current year will be a bug.

// TODO: can following stuff be merged together to avoid duplication?

impl TryFrom<u64> for Junior {
    type Error = Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            9 => Ok(Self::A),
            10 => Ok(Self::B),
            11 => Ok(Self::C),
            12 => Ok(Self::D),
            13 => Ok(Self::E),
            14 => Ok(Self::F),
            15 => Ok(Self::G),
            16 => Ok(Self::H),
            17 => Ok(Self::I),
            18 => Ok(Self::J),
            19 => Ok(Self::K),
            _ => Err(Error::AgeNotJunior),
        }
    }
}

impl TryFrom<isize> for Junior {
    type Error = Error;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            9 => Ok(Self::A),
            10 => Ok(Self::B),
            11 => Ok(Self::C),
            12 => Ok(Self::D),
            13 => Ok(Self::E),
            14 => Ok(Self::F),
            15 => Ok(Self::G),
            16 => Ok(Self::H),
            17 => Ok(Self::I),
            18 => Ok(Self::J),
            19 => Ok(Self::K),
            _ => Err(Error::AgeNotJunior),
        }
    }
}
