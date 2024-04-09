use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter};

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
    Display,
    EnumIter,
    FromPrimitive,
    IntoPrimitive,
    Serialize,
    Deserialize,
)]
#[repr(u32)]
pub enum Specialization {
    #[default]
    Unknown = 0,
    // mesmer
    Chronomancer = 40,
    Mirage = 59,
    Virtuoso = 66,

    // necromancer
    Reaper = 34,
    Scourge = 60,
    Harbinger = 64,

    // revenant
    Herald = 52,
    Renegade = 63,
    Vindicator = 69,

    // warrior
    Berserker = 18,
    Spellbreaker = 61,
    Bladesworn = 68,

    // ranger
    Druid = 5,
    Soulbeast = 55,
    Untamed = 72,

    // engineer
    Scrapper = 43,
    Holosmith = 57,
    Mechanist = 70,

    // thief
    Daredevil = 7,
    Deadeye = 58,
    Specter = 71,

    // guardian
    Dragonhunter = 27,
    Firebrand = 62,
    Willbender = 65,

    // elementalist
    Tempest = 48,
    Weaver = 56,
    Catalyst = 67,
}
