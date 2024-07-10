use crate::{
    colors::{self, Color},
    traits::{Colored, ShortName},
};
use enumflags2::{bitflags, BitFlags};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, IntoStaticStr, VariantArray};

use super::Specialization;

// TODO: transition from bitflags to enum after sufficient grace period for updating

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
#[repr(u16)]
pub enum Profession {
    Guardian = 1 << 1,
    Warrior = 1 << 2,
    Engineer = 1 << 3,
    Ranger = 1 << 4,
    Thief = 1 << 5,
    Elementalist = 1 << 6,
    Mesmer = 1 << 7,
    Necromancer = 1 << 8,
    Revenant = 1 << 9,
}

impl TryFrom<u8> for Profession {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Guardian),
            2 => Ok(Self::Warrior),
            3 => Ok(Self::Engineer),
            4 => Ok(Self::Ranger),
            5 => Ok(Self::Thief),
            6 => Ok(Self::Elementalist),
            7 => Ok(Self::Mesmer),
            8 => Ok(Self::Necromancer),
            9 => Ok(Self::Revenant),
            _ => Err(value),
        }
    }
}

impl Profession {
    pub fn specializations(&self) -> BitFlags<Specialization> {
        match self {
            Self::Guardian => {
                Specialization::Guardian
                    | Specialization::Dragonhunter
                    | Specialization::Firebrand
                    | Specialization::Willbender
            }
            Self::Warrior => {
                Specialization::Warrior
                    | Specialization::Berserker
                    | Specialization::Spellbreaker
                    | Specialization::Bladesworn
            }
            Self::Revenant => {
                Specialization::Revenant
                    | Specialization::Herald
                    | Specialization::Renegade
                    | Specialization::Vindicator
            }
            Self::Engineer => {
                Specialization::Engineer
                    | Specialization::Scrapper
                    | Specialization::Holosmith
                    | Specialization::Mechanist
            }
            Self::Ranger => {
                Specialization::Ranger
                    | Specialization::Druid
                    | Specialization::Soulbeast
                    | Specialization::Untamed
            }
            Self::Thief => {
                Specialization::Thief
                    | Specialization::Daredevil
                    | Specialization::Deadeye
                    | Specialization::Specter
            }
            Self::Elementalist => {
                Specialization::Elementalist
                    | Specialization::Tempest
                    | Specialization::Weaver
                    | Specialization::Catalyst
            }
            Self::Mesmer => {
                Specialization::Mesmer
                    | Specialization::Chronomancer
                    | Specialization::Mirage
                    | Specialization::Virtuoso
            }
            Self::Necromancer => {
                Specialization::Necromancer
                    | Specialization::Reaper
                    | Specialization::Scourge
                    | Specialization::Harbinger
            }
        }
    }
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
