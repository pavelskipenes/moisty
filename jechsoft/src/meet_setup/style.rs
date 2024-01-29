use serde::Deserialize;
use std::{fmt::{self, Display}, convert::TryFrom};

/// `Style` of the `Event`. A wrapper for `Stroke` to account for `Medley`
/// which is a list of `Stroke`s.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

        Self::try_from(deserialized_value.as_ref()).map_err(|_| {
            serde::de::Error::unknown_variant(
                &deserialized_value,
                &[
                    "FREESTYLE",
                    "BUTTERFLY",
                    "BACKSTROKE",
                    "BREASTSTROKE",
                    "INDIVIDUALMEDLEY",
                    "MEDLEYRELAY",
                ],
            )
        })
    }
}

impl TryFrom<&str> for Style {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "FREESTYLE" | "FR" => Ok(Self::Single(Stroke::FreeStyle)),
            "BUTTERFLY" | "BU" => Ok(Self::Single(Stroke::Butterfly)),
            "BACKSTROKE" | "RY" => Ok(Self::Single(Stroke::BackStroke)),
            "BREASTSTROKE" | "BR" => Ok(Self::Single(Stroke::BreastStroke)),
            "INDIVIDUALMEDLEY" | "IM" => Ok(Self::Medley(INDIVIDUAL_MEDLEY)), // this string is present for handicapped individual medley relays as well as regular medley relays
            "MEDLEYRELAY" | "LM" => Ok(Self::Medley(TEAM_MEDLEY)),
            _ => Err(Error::StyleDoesNotExists),
        }
    }
}

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
                    _ => Err(std::fmt::Error),
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

/// Stroke
#[allow(clippy::enum_variant_names)]
#[allow(clippy::module_name_repetitions)]
#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
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

/// Individual medley
pub const INDIVIDUAL_MEDLEY: [Stroke; 4] = [
    Stroke::Butterfly,
    Stroke::BackStroke,
    Stroke::BreastStroke,
    Stroke::FreeStyle,
];

/// Team medley
pub const TEAM_MEDLEY: [Stroke; 4] = [
    Stroke::BackStroke,
    Stroke::BreastStroke,
    Stroke::Butterfly,
    Stroke::FreeStyle,
];

#[derive(Debug, thiserror::Error, Clone, Copy, Deserialize)]
pub enum Error {
    StyleDoesNotExists,
}

#[allow(clippy::recursive_format_impl)]
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match f.align() {
            Some(_) => f.pad(&self.to_string()),
            None => match self {
                Self::StyleDoesNotExists => write!(f, "style does not exists"),
            },
        }
    }
}
