mod ability;
mod buff;
mod skillbar;

pub use self::{ability::*, buff::*, skillbar::*};

use strum::{AsRefStr, Display, IntoStaticStr};

/// Skill identifier.
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
    IntoStaticStr,
)]
pub enum SkillId {
    #[default]
    Unknown,

    #[strum(serialize = "Weapon Swap")]
    WeaponSwap,

    #[strum(serialize = "Bundle Drop")]
    BundleDrop,

    #[strum(serialize = "Pet Swap")]
    PetSwap,

    #[strum(serialize = "{0:>5}")]
    Id(u32),
}

impl From<u32> for SkillId {
    #[inline]
    fn from(id: u32) -> Self {
        Self::Id(id)
    }
}

/// Information about a skill.
#[derive(Debug, Clone)]
pub enum SkillInfo {
    /// Ability.
    Ability {
        /// Whether the ability is instant cast.
        is_instant: bool,

        /// Whether the ability has ammunition.
        is_ammo: bool,

        /// Whether the ability is ground targeted.
        is_ground_targeted: bool,

        /// Whether the ability is a stunbreak.
        is_stunbreak: bool,
    },

    /// Buff.
    Buff {
        /// Category of the buff.
        category: Category,

        /// Stacking behavior of the buff.
        stacking: Stacking,
    },
}
