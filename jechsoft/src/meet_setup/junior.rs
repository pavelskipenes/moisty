#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cannot create a junior class outside of valid age range 9..19 ")]
    OutOfRange,
}

#[derive(Debug)]
pub enum Junior {
    A = 9,
    B = 10,
    C = 11,
    D = 12,
    E = 13,
    F = 14,
    G = 15,
    H = 16,
    I = 17,
    J = 18,
    K = 19,
}

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
            _ => Err(Error::OutOfRange),
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
            _ => Err(Error::OutOfRange),
        }
    }
}
