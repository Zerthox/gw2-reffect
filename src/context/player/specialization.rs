use super::Profession;
use crate::{colors::Color, traits::Colored};
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
#[repr(u32)]
pub enum Specialization {
    // guardian
    Dragonhunter = 27,
    Firebrand = 62,
    Willbender = 65,

    // warrior
    Berserker = 18,
    Spellbreaker = 61,
    Bladesworn = 68,

    // revenant
    Herald = 52,
    Renegade = 63,
    Vindicator = 69,

    // engineer
    Scrapper = 43,
    Holosmith = 57,
    Mechanist = 70,

    // ranger
    Druid = 5,
    Soulbeast = 55,
    Untamed = 72,

    // thief
    Daredevil = 7,
    Deadeye = 58,
    Specter = 71,

    // elementalist
    Tempest = 48,
    Weaver = 56,
    Catalyst = 67,

    // mesmer
    Chronomancer = 40,
    Mirage = 59,
    Virtuoso = 66,

    // necromancer
    Reaper = 34,
    Scourge = 60,
    Harbinger = 64,
}

impl Specialization {
    pub fn profession(&self) -> Profession {
        match self {
            Self::Dragonhunter | Self::Firebrand | Self::Willbender => Profession::Guardian,
            Self::Berserker | Self::Spellbreaker | Self::Bladesworn => Profession::Warrior,
            Self::Herald | Self::Renegade | Self::Vindicator => Profession::Revenant,
            Self::Scrapper | Self::Holosmith | Self::Mechanist => Profession::Engineer,
            Self::Druid | Self::Soulbeast | Self::Untamed => Profession::Ranger,
            Self::Daredevil | Self::Deadeye | Self::Specter => Profession::Thief,
            Self::Tempest | Self::Weaver | Self::Catalyst => Profession::Elementalist,
            Self::Chronomancer | Self::Mirage | Self::Virtuoso => Profession::Mesmer,
            Self::Reaper | Self::Scourge | Self::Harbinger => Profession::Necromancer,
        }
    }
}

impl Colored for Specialization {
    fn colored(&self) -> Option<Color> {
        self.profession().colored()
    }
}
