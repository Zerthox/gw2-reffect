use crate::{
    colors::{self, Color},
    traits::{Colored, ShortName},
};
use num_enum::{IntoPrimitive, TryFromPrimitive};
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
    TryFromPrimitive,
    IntoPrimitive,
)]
#[repr(u8)]
pub enum Profession {
    Guardian = 1,
    Warrior = 2,
    Engineer = 3,
    Ranger = 4,
    Thief = 5,
    Elementalist = 6,
    Mesmer = 7,
    Necromancer = 8,
    Revenant = 9,
}

impl ShortName for Profession {
    fn short_name(&self) -> &'static str {
        match self {
            Self::Guardian => "Gdn",
            Self::Warrior => "War",
            Self::Revenant => "Rev",
            Self::Engineer => "Eng",
            Self::Ranger => "Rgr",
            Self::Thief => "Thf",
            Self::Elementalist => "Ele",
            Self::Mesmer => "Mes",
            Self::Necromancer => "Nec",
        }
    }
}

impl Colored for Profession {
    fn colored(&self) -> Option<Color> {
        Some(match self {
            Self::Guardian => colors::GUARDIAN,
            Self::Warrior => colors::WARRIOR,
            Self::Revenant => colors::REVENANT,
            Self::Engineer => colors::ENGINEER,
            Self::Ranger => colors::RANGER,
            Self::Thief => colors::THIEF,
            Self::Elementalist => colors::ELEMENTALIST,
            Self::Mesmer => colors::MESMER,
            Self::Necromancer => colors::NECROMANCER,
        })
    }
}
