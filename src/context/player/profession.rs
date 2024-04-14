use crate::{
    colors::{self, Color},
    traits::Colored,
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
    TryFromPrimitive,
    IntoPrimitive,
    Serialize,
    Deserialize,
)]
#[repr(u8)]
pub enum Profession {
    Guardian = 1,
    Warrior = 2,
    Revenant = 9,
    Engineer = 3,
    Ranger = 4,
    Thief = 5,
    Elementalist = 6,
    Mesmer = 7,
    Necromancer = 8,
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
