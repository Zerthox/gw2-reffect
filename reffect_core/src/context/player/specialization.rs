use super::Profession;
use crate::{
    colors::{Color, Colored},
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
#[repr(u64)]
pub enum Specialization {
    #[strum(serialize = "Guardian (core)")]
    Guardian = 1 << 0,
    Dragonhunter = 1 << 1,
    Firebrand = 1 << 2,
    Willbender = 1 << 3,
    Luminary = 1 << 4,

    #[strum(serialize = "Warrior (core)")]
    Warrior = 1 << 5,
    Berserker = 1 << 6,
    Spellbreaker = 1 << 7,
    Bladesworn = 1 << 8,
    Paragon = 1 << 9,

    #[strum(serialize = "Revenant (core)")]
    Revenant = 1 << 10,
    Herald = 1 << 11,
    Renegade = 1 << 12,
    Vindicator = 1 << 13,
    Conduit = 1 << 14,

    #[strum(serialize = "Engineer (core)")]
    Engineer = 1 << 15,
    Scrapper = 1 << 16,
    Holosmith = 1 << 17,
    Mechanist = 1 << 18,
    Amalgam = 1 << 19,

    #[strum(serialize = "Ranger (core)")]
    Ranger = 1 << 20,
    Druid = 1 << 21,
    Soulbeast = 1 << 22,
    Untamed = 1 << 23,
    Galeshot = 1 << 24,

    #[strum(serialize = "Thief (core)")]
    Thief = 1 << 25,
    Daredevil = 1 << 26,
    Deadeye = 1 << 27,
    Specter = 1 << 28,
    Antiquary = 1 << 29,

    #[strum(serialize = "Elementalist (core)")]
    Elementalist = 1 << 30,
    Tempest = 1 << 31,
    Weaver = 1 << 32,
    Catalyst = 1 << 33,
    Evoker = 1 << 34,

    #[strum(serialize = "Mesmer (core)")]
    Mesmer = 1 << 35,
    Chronomancer = 1 << 36,
    Mirage = 1 << 37,
    Virtuoso = 1 << 38,
    Troubadour = 1 << 39,

    #[strum(serialize = "Necromancer (core)")]
    Necromancer = 1 << 40,
    Reaper = 1 << 41,
    Scourge = 1 << 42,
    Harbinger = 1 << 43,
    Ritualist = 1 << 44,
}

impl Specialization {
    pub fn try_from(prof: Option<Profession>, spec: u32) -> Option<Self> {
        match spec {
            5 => Some(Self::Druid),
            7 => Some(Self::Daredevil),
            18 => Some(Self::Berserker),
            27 => Some(Self::Dragonhunter),
            34 => Some(Self::Reaper),
            40 => Some(Self::Chronomancer),
            43 => Some(Self::Scrapper),
            48 => Some(Self::Tempest),
            52 => Some(Self::Herald),
            55 => Some(Self::Soulbeast),
            56 => Some(Self::Weaver),
            57 => Some(Self::Holosmith),
            58 => Some(Self::Deadeye),
            59 => Some(Self::Mirage),
            60 => Some(Self::Scourge),
            61 => Some(Self::Spellbreaker),
            62 => Some(Self::Firebrand),
            63 => Some(Self::Renegade),
            64 => Some(Self::Harbinger),
            65 => Some(Self::Willbender),
            66 => Some(Self::Virtuoso),
            67 => Some(Self::Catalyst),
            68 => Some(Self::Bladesworn),
            69 => Some(Self::Vindicator),
            70 => Some(Self::Mechanist),
            71 => Some(Self::Specter),
            72 => Some(Self::Untamed),
            73 => Some(Self::Troubadour),
            74 => Some(Self::Paragon),
            75 => Some(Self::Amalgam),
            76 => Some(Self::Ritualist),
            77 => Some(Self::Antiquary),
            78 => Some(Self::Galeshot),
            79 => Some(Self::Conduit),
            80 => Some(Self::Evoker),
            81 => Some(Self::Luminary),
            _ => prof.map(|prof| prof.into()),
        }
    }

    pub const fn profession(&self) -> Profession {
        match self {
            Self::Guardian
            | Self::Dragonhunter
            | Self::Firebrand
            | Self::Willbender
            | Self::Luminary => Profession::Guardian,

            Self::Warrior
            | Self::Berserker
            | Self::Spellbreaker
            | Self::Bladesworn
            | Self::Paragon => Profession::Warrior,

            Self::Revenant | Self::Herald | Self::Renegade | Self::Vindicator | Self::Conduit => {
                Profession::Revenant
            }

            Self::Engineer | Self::Scrapper | Self::Holosmith | Self::Mechanist | Self::Amalgam => {
                Profession::Engineer
            }

            Self::Ranger | Self::Druid | Self::Soulbeast | Self::Untamed | Self::Galeshot => {
                Profession::Ranger
            }

            Self::Thief | Self::Daredevil | Self::Deadeye | Self::Specter | Self::Antiquary => {
                Profession::Thief
            }

            Self::Elementalist | Self::Tempest | Self::Weaver | Self::Catalyst | Self::Evoker => {
                Profession::Elementalist
            }

            Self::Mesmer
            | Self::Chronomancer
            | Self::Mirage
            | Self::Virtuoso
            | Self::Troubadour => Profession::Mesmer,

            Self::Necromancer
            | Self::Reaper
            | Self::Scourge
            | Self::Harbinger
            | Self::Ritualist => Profession::Necromancer,
        }
    }
}

impl From<Profession> for Specialization {
    fn from(prof: Profession) -> Self {
        match prof {
            Profession::Guardian => Self::Guardian,
            Profession::Warrior => Self::Warrior,
            Profession::Engineer => Self::Engineer,
            Profession::Ranger => Self::Ranger,
            Profession::Thief => Self::Thief,
            Profession::Elementalist => Self::Elementalist,
            Profession::Mesmer => Self::Mesmer,
            Profession::Necromancer => Self::Necromancer,
            Profession::Revenant => Self::Revenant,
        }
    }
}

impl Named for Specialization {
    fn name(&self) -> &'static str {
        self.into()
    }

    fn short_name(&self) -> &'static str {
        match self {
            Self::Guardian => "Gdn",
            Self::Dragonhunter => "Dgh",
            Self::Firebrand => "Fbd",
            Self::Willbender => "Wbd",
            Self::Luminary => "Lum",

            Self::Warrior => "War",
            Self::Berserker => "Brs",
            Self::Spellbreaker => "Spb",
            Self::Bladesworn => "Bsw",
            Self::Paragon => "Par",

            Self::Revenant => "Rev",
            Self::Herald => "Her",
            Self::Renegade => "Ren",
            Self::Vindicator => "Vin",
            Self::Conduit => "Con",

            Self::Engineer => "Eng",
            Self::Scrapper => "Scr",
            Self::Holosmith => "Hls",
            Self::Mechanist => "Mec",
            Self::Amalgam => "Ama",

            Self::Ranger => "Rgr",
            Self::Druid => "Dru",
            Self::Soulbeast => "Slb",
            Self::Untamed => "Unt",
            Self::Galeshot => "Gls",

            Self::Thief => "Thf",
            Self::Daredevil => "Dar",
            Self::Deadeye => "Ded",
            Self::Specter => "Spe",
            Self::Antiquary => "Ant",

            Self::Elementalist => "Ele",
            Self::Tempest => "Tmp",
            Self::Weaver => "Wea",
            Self::Catalyst => "Cat",
            Self::Evoker => "Evo",

            Self::Mesmer => "Mes",
            Self::Chronomancer => "Chr",
            Self::Mirage => "Mir",
            Self::Virtuoso => "Vir",
            Self::Troubadour => "Trb",

            Self::Necromancer => "Nec",
            Self::Reaper => "Rea",
            Self::Scourge => "Scg",
            Self::Harbinger => "Har",
            Self::Ritualist => "Rit",
        }
    }
}

impl Colored for Specialization {
    fn colored(&self) -> Option<Color> {
        self.profession().colored()
    }
}
