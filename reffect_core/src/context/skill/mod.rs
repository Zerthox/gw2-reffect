mod ability;
mod buff;

pub use self::{ability::*, buff::*};

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
        /// Whether the ability is an ammunition skill.
        is_ammo: bool,
    },

    /// Buff.
    Buff {
        /// Category of the buff.
        category: Category,

        /// Stacking behavior of the buff.
        stacking: Stacking,
    },
}

/// Category of the buff.
///
/// Any category except for Boon and Condition is mapped to [`Category::Effect`].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, AsRefStr, IntoStaticStr,
)]
pub enum Category {
    /// Buff is a Boon.
    Boon,

    /// Buff is an uncategorized effect.
    Effect,

    /// Buff is a Condition.
    Condition,

    /// Buff is hidden but gives a screen border.
    #[strum(serialize = "Screen Border")]
    ScreenBorder,

    /// Buff is hidden but highlights player in squad.
    #[strum(serialize = "Squad Highlight")]
    SquadHighlight,
}

/// Stacking behavior of the buff.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, AsRefStr, IntoStaticStr,
)]
pub enum Stacking {
    // Other/unknown stacking type.
    Other,

    /// Buff stacks in intenstity.
    Intensity,

    /// Buff stacks in duration.
    Duration,
}
