use serde::{ser::Error, Deserialize};
use std::fmt;

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
#[allow(clippy::enum_variant_names)]
#[allow(clippy::module_name_repetitions)]

/// Stroke
pub enum Stroke {
    /// Backstroke.
    BackStroke,
    /// Breaststroke.
    BreastStroke,
    /// Freestyle.
    FreeStyle,
    /// Butterfly.
    Butterfly,
}

/// `Style` of the `Event`. A wrapper for `Stroke` to account for `Medley`
/// which is a list of `Stroke`s.
#[derive(Debug)]
pub enum Style {
    /// Single stroke.
    Single(Stroke),
    /// Four styles combined in one event. See `INDIVIDUAL_MEDLEY` and `TEAM_MEDLEY`.
    Medley([Stroke; 4]),
    // Cannot deserialize following structure because it is indistinguishable from individual medley.
    // Three styles combined in one event. Only valid for some HC groups. `INDIVIDUAL_MEDLEY_HC`.
    // MedleyHC([Stroke; 3]),
}

impl<'de> Deserialize<'de> for Style {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let deserialized_value: String = Deserialize::deserialize(deserializer)?;

        // TODO: add hc medley variant
        match deserialized_value.as_ref() {
            "FREESTYLE" => Ok(Self::Single(Stroke::FreeStyle)),
            "BUTTERFLY" => Ok(Self::Single(Stroke::Butterfly)),
            "BACKSTROKE" => Ok(Self::Single(Stroke::BackStroke)),
            "BREASTSTROKE" => Ok(Self::Single(Stroke::BreastStroke)),
            "INDIVIDUALMEDLEY" => Ok(Self::Medley(INDIVIDUAL_MEDLEY)), // this string is present for handicapped individual medley relays as well as regular medley relays
            "MEDLEYRELAY" => Ok(Self::Medley(TEAM_MEDLEY)),
            _ => Err(serde::de::Error::unknown_variant(
                &deserialized_value,
                &[
                    "FREESTYLE",
                    "BUTTERFLY",
                    "BACKSTROKE",
                    "BREASTSTROKE",
                    "INDIVIDUALMEDLEY",
                    "MEDLEYRELAY",
                ],
            )),
        }
    }
}

/// Individual medley with styles in order
pub const INDIVIDUAL_MEDLEY: [Stroke; 4] = [
    Stroke::Butterfly,
    Stroke::BackStroke,
    Stroke::BreastStroke,
    Stroke::FreeStyle,
];

/// Team medley with it's styles in order
pub const TEAM_MEDLEY: [Stroke; 4] = [
    Stroke::BackStroke,
    Stroke::BreastStroke,
    Stroke::Butterfly,
    Stroke::FreeStyle,
];

// pub const INDIVIDUAL_MEDLEY_HC: [Stroke; 3] =
//     [Stroke::BackStroke, Stroke::BreastStroke, Stroke::FreeStyle];

#[allow(clippy::recursive_format_impl)]
impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match f.align() {
            None => match self {
                Self::Medley(medley) => match *medley {
                    TEAM_MEDLEY => write!(f, "team medley"),
                    INDIVIDUAL_MEDLEY => write!(f, "individual medley"),
                    _ => Err(fmt::Error::custom("Not a valid medley order")),
                },
                Self::Single(a) => match *a {
                    Stroke::BackStroke => write!(f, "backstroke"),
                    Stroke::FreeStyle => write!(f, "freestyle"),
                    Stroke::Butterfly => write!(f, "butterfly"),
                    Stroke::BreastStroke => write!(f, "breaststroke"),
                },
                // Style::MedleyHC(medley_hc) => match *medley_hc {
                //     INDIVIDUAL_MEDLEY_HC => write!(f, "handicap medley"),
                //     _ => Err(fmt::Error::custom("Not a valid handicap medley order")),
                // },
            },
            Some(_) => f.pad(&self.to_string()),
        }
    }
}
