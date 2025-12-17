use enumflags2::{BitFlags, bitflags};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, IntoStaticStr, VariantArray};

use crate::{
    colors::{self, Color, Colored},
    named::Named,
};

#[derive(Debug, Clone)]
pub struct Build {
    /// Selected specializations.
    pub specs: Specializations,

    /// Selected traits.
    pub traits: Traits,

    /// Selected skills.
    pub selected_skills: Vec<u32>,

    /// Profession-specific info.
    pub prof_info: BitFlags<ProfInfo>,
}

impl Build {
    #[inline]
    pub const fn empty() -> Self {
        Self {
            specs: [0; 3],
            traits: [0; 9],
            selected_skills: Vec::new(),
            prof_info: BitFlags::EMPTY,
        }
    }
}

pub type Specializations = [u32; 3];

pub type Traits = [u32; 9];

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
pub enum ProfInfo {
    #[strum(serialize = "Assassin")]
    LegendAssassin = 1 << 0,

    #[strum(serialize = "Centaur")]
    LegendCentaur = 1 << 1,

    #[strum(serialize = "Demon")]
    LegendDemon = 1 << 2,

    #[strum(serialize = "Dwarf")]
    LegendDwarf = 1 << 3,

    #[strum(serialize = "Dragon")]
    LegendDragon = 1 << 4,

    #[strum(serialize = "Renegade")]
    LegendRenegade = 1 << 5,

    #[strum(serialize = "Alliance")]
    LegendAlliance = 1 << 6,

    #[strum(serialize = "Entity")]
    LegendEntity = 1 << 7,

    #[strum(serialize = "Fox")]
    FamiliarFire = 1 << 8,

    #[strum(serialize = "Otter")]
    FamiliarWater = 1 << 9,

    #[strum(serialize = "Hare")]
    FamiliarAir = 1 << 10,

    #[strum(serialize = "Toad")]
    FamiliarEarth = 1 << 11,
}

impl Named for ProfInfo {
    #[inline]
    fn name(&self) -> &'static str {
        self.into()
    }
}

impl Colored for ProfInfo {
    #[inline]
    fn colored(&self) -> Option<Color> {
        match self {
            Self::LegendAssassin
            | Self::LegendCentaur
            | Self::LegendDemon
            | Self::LegendDwarf
            | Self::LegendDragon
            | Self::LegendRenegade
            | Self::LegendAlliance
            | Self::LegendEntity => Some(colors::REVENANT),
            Self::FamiliarFire | Self::FamiliarWater | Self::FamiliarAir | Self::FamiliarEarth => {
                Some(colors::ELEMENTALIST)
            }
        }
    }
}
