use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, IntoStaticStr, VariantArray};

use crate::{
    colors::{self, Color},
    traits::Colored,
};

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    AsRefStr,
    IntoStaticStr,
    Display,
    EnumIter,
    VariantArray,
    FromPrimitive,
    IntoPrimitive,
    Serialize,
    Deserialize,
)]
#[repr(u8)]
pub enum Mount {
    #[default]
    None = 0,

    Raptor = 5,

    Springer = 3,

    Skimmer = 4,

    Jackal = 1,

    Griffon = 2,

    #[strum(serialize = "Roller Beetle")]
    RollerBeetle = 6,

    Warclaw = 7,

    Skyscale = 8,

    #[strum(serialize = "Siege Turtle")]
    SiegeTurtle = 10,

    Skiff = 9,
}

// TODO: mount colors
impl Colored for Mount {
    fn colored(&self) -> Option<Color> {
        match self {
            Self::None => None,
            Self::Jackal => Some(colors::JACKAL),
            Self::Griffon => Some(colors::GRIFFON),
            Self::Springer => Some(colors::SPRINGER),
            Self::Skimmer => Some(colors::SKIMMER),
            Self::Raptor => Some(colors::RAPTOR),
            Self::RollerBeetle => Some(colors::ROLLER_BEETLE),
            Self::Warclaw => Some(colors::WARCLAW),
            Self::Skyscale => Some(colors::SKYSCALE),
            Self::SiegeTurtle => Some(colors::SIEGE_TURTLE),
            Self::Skiff => None,
        }
    }
}
