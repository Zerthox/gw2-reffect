use super::{Ability, SkillId};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumCount, EnumIter, IntoStaticStr, VariantArray};

pub type SkillSlots = [Option<Ability>; Slot::COUNT];

/// Character skillbar.
#[derive(Debug, Default, Clone)]
pub struct Skillbar {
    /// Skill entries.
    pub skills: SkillSlots,
}

impl Skillbar {
    /// Returns the ability in the given slot.
    #[inline]
    pub fn slot(&self, slot: Slot) -> Option<&Ability> {
        self.skills[slot as usize].as_ref()
    }

    /// Returns the ability with the given identifier.
    #[inline]
    pub fn ability(&self, id: impl Into<SkillId>) -> Option<&Ability> {
        let id = id.into();
        self.skills.iter().flatten().find(|ablity| ablity.id == id)
    }

    /// Sets the ability in the given slot.
    #[inline]
    pub fn set_slot(&mut self, slot: Slot, ability: Option<Ability>) {
        self.skills[slot as usize] = ability;
    }
}

/// Skillbar slot.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Display,
    AsRefStr,
    IntoStaticStr,
    EnumCount,
    EnumIter,
    VariantArray,
    Serialize,
    Deserialize,
)]
pub enum Slot {
    #[strum(serialize = "Weapon Swap")]
    WeaponSwap,

    #[strum(serialize = "Weapon 1")]
    Weapon1,

    #[strum(serialize = "Weapon 2")]
    Weapon2,

    #[strum(serialize = "Weapon 3")]
    Weapon3,

    #[strum(serialize = "Weapon 4")]
    Weapon4,

    #[strum(serialize = "Weapon 5")]
    Weapon5,

    Heal,

    #[strum(serialize = "Utility 1")]
    Utility1,

    #[strum(serialize = "Utility 2")]
    Utility2,

    #[strum(serialize = "Utility 3")]
    Utility3,

    Elite,

    #[strum(serialize = "Profession 1")]
    Profession1,

    #[strum(serialize = "Profession 2")]
    Profession2,

    #[strum(serialize = "Profession 3")]
    Profession3,

    #[strum(serialize = "Profession 4")]
    Profession4,

    #[strum(serialize = "Profession 5")]
    Profession5,

    #[strum(serialize = "Special Action")]
    SpecialAction,

    Mount,
}

impl Slot {
    pub const DEFAULT: Self = Self::WeaponSwap;
}

impl Default for Slot {
    fn default() -> Self {
        Self::DEFAULT
    }
}
