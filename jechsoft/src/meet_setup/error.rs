/// Errors in crate
pub enum Error {
    /// Competition type does not exists
    NonExistingCompetitionType,

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
