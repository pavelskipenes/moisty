use std::fmt::Display;

/// Errors in crate
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Invalid distance
    InvalidDistance,

    /// Attempting to construct a Junior but the age is outside junior class. Use Senior or something like that.
    NotAJuniorAge,

    /// Attempting to deserialize a class for junior team relay given a birth year. Team relays dont have birth years and are only specified as junior `"JR"`
    TeamRelaysDoesNotHaveJuniorClassGroup,

    /// This method is only valid for Junior variants.
    NotAJuniorVariant,

    /// Unknown competition type id
    CompetitionTypeIdDoesNotExists,

    /// Too many variants with given length exists. Construct the variant manually
    IndistinguishableDistance,

    /// given age is not in a valid Junior range
    AgeNotJunior,
}

#[allow(clippy::recursive_format_impl)]
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match f.align() {
            Some(_) => f.pad(&self.to_string()),
            None => match self {
                Self::InvalidDistance => write!(f, "invalid distance"),
                Self::NotAJuniorAge => write!(f, "attempting to construct a junior but the age is not a junior age"),
                Self::TeamRelaysDoesNotHaveJuniorClassGroup => write!(f, "team relays does not have an associated class year"),
                Self::NotAJuniorVariant => write!(f, "cannot convert class into Junior because this class is not of Junior variant"),
                Self::CompetitionTypeIdDoesNotExists => write!(f, "competition type does not exists"),
                Self::IndistinguishableDistance => write!(f, "cannot determine the distance from the given string. Too many distances can be constructed"),
                Self::AgeNotJunior => write!(f, "age is outside valid junior range"),
            },
        }
    }
}

