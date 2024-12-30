use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumCount, EnumIter, IntoStaticStr, VariantArray};

pub type SkillSlots = [Option<Ability>; Slot::COUNT];

/// Character skillbar.
#[derive(Debug, Default, Clone)]
pub struct Skillbar {
    /// Whether the player is carrying a bundle.
    pub has_bundle: bool,

    /// Skill entries.
    pub skills: SkillSlots,
}

impl Skillbar {
    /// Returns the ability in the given slot.
    #[inline]
    pub fn slot(&self, slot: Slot) -> Option<&Ability> {
        self.skills[slot as usize].as_ref()
    }

    /// Returns the ability with the given id.
    #[inline]
    pub fn ability(&self, id: u32) -> Option<&Ability> {
        self.skills.iter().flatten().find(|ablity| ablity.id == id)
    }

    /// Sets the ability in the given slot.
    #[inline]
    pub fn set_slot(&mut self, slot: Slot, ability: Option<Ability>) {
        self.skills[slot as usize] = ability;
    }
}

/// Ability.
#[derive(Debug, Clone)]
pub struct Ability {
    /// Skill id.
    pub id: u32,

    /// Ammunition count.
    pub ammo: u32,

    /// Last update timestamp.
    pub last_update: u32,

    /// Recharge rate.
    pub recharge_rate: f32,

    /// Total recharge in milliseconds.
    pub recharge: u32,

    /// Remaining recharge in milliseconds.
    pub recharge_remaining: u32,

    /// Total ammo recharge in milliseconds.
    pub ammo_recharge: u32,

    /// Remining ammo recharge in milliseconds.
    pub ammo_recharge_remaining: u32,
}

impl Ability {
    /// Creates a new ability without cooldowns.
    #[inline]
    pub const fn available(id: u32, ammo: u32, last_update: u32, recharge_rate: f32) -> Self {
        Self {
            id,
            ammo,
            last_update,
            recharge_rate,
            recharge: 0,
            recharge_remaining: 0,
            ammo_recharge: 0,
            ammo_recharge_remaining: 0,
        }
    }

    /// Creates a new ability with simple recharge.
    #[inline]
    pub const fn simple(id: u32, last_update: u32, recharge: u32) -> Self {
        Self {
            id,
            ammo: if recharge == 0 { 1 } else { 0 },
            last_update,
            recharge_rate: 1.0,
            recharge,
            recharge_remaining: recharge,
            ammo_recharge: 0,
            ammo_recharge_remaining: 0,
        }
    }

    /// Returns the scaled amount of time passed since the last update.
    #[inline]
    pub fn passed(&self, now: u32) -> u32 {
        let passed = now.saturating_sub(self.last_update);
        (passed as f32 * self.recharge_rate) as u32
    }

    /// Returns the remaining recharge.
    #[inline]
    pub fn recharge_remaining(&self, now: u32) -> u32 {
        self.recharge_remaining.saturating_sub(self.passed(now))
    }

    /// Returns the recharge progress.
    #[inline]
    pub fn recharge_progress(&self, now: u32) -> f32 {
        self.recharge_remaining(self.passed(now)) as f32 / self.recharge as f32
    }

    /// Returns the remaining ammo recharge.
    #[inline]
    pub fn ammo_recharge_remaining(&self, now: u32) -> u32 {
        self.ammo_recharge_remaining
            .saturating_sub(self.passed(now))
    }

    /// Returns the ammo recharge progress.
    #[inline]
    pub fn ammo_recharge_progress(&self, now: u32) -> f32 {
        self.ammo_recharge_remaining(self.passed(now)) as f32 / self.ammo_recharge as f32
    }
}

/// Skillbar slot.
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
    #[default]
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
