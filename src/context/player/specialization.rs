use super::Profession;
use crate::{
    colors::Color,
    traits::{Colored, ShortName},
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
#[bitflags]
#[repr(u32)]
pub enum Specialization {
    // guardian
    Dragonhunter = 1 << 1,
    Firebrand = 1 << 2,
    Willbender = 1 << 3,

    // warrior
    Berserker = 1 << 4,
    Spellbreaker = 1 << 5,
    Bladesworn = 1 << 6,

    // revenant
    Herald = 1 << 7,
    Renegade = 1 << 8,
    Vindicator = 1 << 9,

    // engineer
    Scrapper = 1 << 10,
    Holosmith = 1 << 11,
    Mechanist = 1 << 12,

    // ranger
    Druid = 1 << 13,
    Soulbeast = 1 << 14,
    Untamed = 1 << 15,

    // thief
    Daredevil = 1 << 16,
    Deadeye = 1 << 17,
    Specter = 1 << 18,

    // elementalist
    Tempest = 1 << 19,
    Weaver = 1 << 20,
    Catalyst = 1 << 21,

    // mesmer
    Chronomancer = 1 << 22,
    Mirage = 1 << 23,
    Virtuoso = 1 << 24,

    // necromancer
    Reaper = 1 << 25,
    Scourge = 1 << 26,
    Harbinger = 1 << 27,
}

impl TryFrom<u32> for Specialization {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            5 => Ok(Self::Druid),
            7 => Ok(Self::Daredevil),
            18 => Ok(Self::Berserker),
            27 => Ok(Self::Dragonhunter),
            34 => Ok(Self::Reaper),
            40 => Ok(Self::Chronomancer),
            43 => Ok(Self::Scrapper),
            48 => Ok(Self::Tempest),
            52 => Ok(Self::Herald),
            55 => Ok(Self::Soulbeast),
            56 => Ok(Self::Weaver),
            57 => Ok(Self::Holosmith),
            58 => Ok(Self::Deadeye),
            59 => Ok(Self::Mirage),
            60 => Ok(Self::Scourge),
            61 => Ok(Self::Spellbreaker),
            62 => Ok(Self::Firebrand),
            63 => Ok(Self::Renegade),
            64 => Ok(Self::Harbinger),
            65 => Ok(Self::Willbender),
            66 => Ok(Self::Virtuoso),
            67 => Ok(Self::Catalyst),
            68 => Ok(Self::Bladesworn),
            69 => Ok(Self::Vindicator),
            70 => Ok(Self::Mechanist),
            71 => Ok(Self::Specter),
            72 => Ok(Self::Untamed),
            _ => Err(value),
        }
    }
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

impl ShortName for Specialization {
    fn short_name(&self) -> &'static str {
        match self {
            Self::Dragonhunter => "Dgh",
            Self::Firebrand => "Fbd",
            Self::Willbender => "Wbd",
            Self::Berserker => "Brs",
            Self::Spellbreaker => "Spb",
            Self::Bladesworn => "Bls",
            Self::Herald => "Her",
            Self::Renegade => "Ren",
            Self::Vindicator => "Vin",
            Self::Scrapper => "Scr",
            Self::Holosmith => "Hls",
            Self::Mechanist => "Mec",
            Self::Druid => "Dru",
            Self::Soulbeast => "Slb",
            Self::Untamed => "Unt",
            Self::Daredevil => "Dar",
            Self::Deadeye => "Ded",
            Self::Specter => "Spe",
            Self::Tempest => "Tmp",
            Self::Weaver => "Wea",
            Self::Catalyst => "Cat",
            Self::Chronomancer => "Chr",
            Self::Mirage => "Mir",
            Self::Virtuoso => "Vir",
            Self::Reaper => "Rea",
            Self::Scourge => "Scg",
            Self::Harbinger => "Har",
        }
    }
}

impl Colored for Specialization {
    fn colored(&self) -> Option<Color> {
        self.profession().colored()
    }
}
