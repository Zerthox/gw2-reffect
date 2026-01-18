mod ability;
mod buff;
mod skillbar;

pub use self::{ability::*, buff::*, skillbar::*};

use enumflags2::{BitFlags, bitflags};
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

        /// Visibility of the buff.
        visibility: BitFlags<Visibility>,
    },
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, AsRefStr, IntoStaticStr, Display,
)]
#[repr(u8)]
#[bitflags]
pub enum Visibility {
    /// Visible for player character.
    Player = 1 << 0,

    /// Visible for non-hostile target.
    TargetNonHostile = 1 << 1,

    /// Visible for hostile target.
    TargetHostile = 1 << 2,

    /// Visible for group member.
    Group = 1 << 3,
}

impl Visibility {
    /// Whether the visibility allows full durations for boons & conditions.
    #[inline]
    pub const fn allow_full_durations(&self) -> bool {
        matches!(
            self,
            Self::Player | Self::TargetHostile | Self::TargetNonHostile,
        )
    }
}
