use crate::{
    colors::{self, Color, Colored},
    named::Named,
};
use enumflags2::bitflags;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, IntoStaticStr, VariantArray};

#[derive(
    Debug,
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
    Serialize,
    Deserialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[bitflags]
#[repr(u32)]
pub enum Mount {
    None = 1 << 1,

    Raptor = 1 << 2,

    Springer = 1 << 3,

    Skimmer = 1 << 4,

    Jackal = 1 << 5,

    Griffon = 1 << 6,

    #[strum(serialize = "Roller Beetle")]
    RollerBeetle = 1 << 7,

    Warclaw = 1 << 8,

    Skyscale = 1 << 9,

    #[strum(serialize = "Siege Turtle")]
    SiegeTurtle = 1 << 10,

    Skiff = 1 << 11,
}

impl TryFrom<u8> for Mount {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Jackal),
            2 => Ok(Self::Griffon),
            3 => Ok(Self::Springer),
            4 => Ok(Self::Skimmer),
            5 => Ok(Self::Raptor),
            6 => Ok(Self::RollerBeetle),
            7 => Ok(Self::Warclaw),
            8 => Ok(Self::Skyscale),
            9 => Ok(Self::Skiff),
            10 => Ok(Self::SiegeTurtle),
            _ => Err(value),
        }
    }
}

impl Named for Mount {
    #[inline]
    fn name(&self) -> &'static str {
        self.into()
    }

    #[inline]
    fn short_name(&self) -> &'static str {
        match self {
            Self::None => "No",
            Self::Jackal => "Jkl",
            Self::Griffon => "Grf",
            Self::Springer => "Spr",
            Self::Skimmer => "Skm",
            Self::Raptor => "Rpt",
            Self::RollerBeetle => "Btl",
            Self::Warclaw => "Wcl",
            Self::Skyscale => "Sky",
            Self::SiegeTurtle => "Ttl",
            Self::Skiff => "Skf",
        }
    }
}

impl Colored for Mount {
    #[inline]
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
