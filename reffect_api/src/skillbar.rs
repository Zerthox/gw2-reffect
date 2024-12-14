use strum::{AsRefStr, Display, EnumCount, EnumIter, IntoStaticStr};

pub type SkillSlots = [Option<Ability>; Slot::COUNT];

/// Character skillbar.
#[derive(Debug, Default, Clone)]
pub struct Skillbar {
    /// Last update timestamp.
    pub last_update: u32,

    /// Recharge rate.
    pub recharge_rate: f32,

    /// Skill entries.
    pub skills: SkillSlots,

    /// Weapon swap.
    pub weapon_swap: Option<Recharge>,

    /// Revenant legend swap.
    pub legend_swap: Option<Recharge>,
}

impl Skillbar {
    /// Returns the scaled amount of time passed since the last update.
    #[inline]
    pub fn passed(&self, now: u32) -> u32 {
        let passed = now.saturating_sub(self.last_update);
        (passed as f32 * self.recharge_rate) as u32
    }

    /// Returns the ability in the given slot.
    #[inline]
    pub fn slot(&self, slot: Slot) -> Option<&Ability> {
        self.skills[slot as usize].as_ref()
    }
}

/// Ability.
#[derive(Debug, Clone)]
pub struct Ability {
    /// Skill id.
    pub id: u32,
    /// Ammunition count.
    pub ammo: u32,

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
    #[inline]
    pub const fn new(id: u32, ammo: u32) -> Self {
        Self {
            id,
            ammo,
            recharge: 0,
            recharge_remaining: 0,
            ammo_recharge: 0,
            ammo_recharge_remaining: 0,
        }
    }

    /// Returns the remaining recharge.
    #[inline]
    pub fn recharge_remaining(&self, passed: u32) -> u32 {
        self.recharge_remaining.saturating_sub(passed)
    }

    /// Returns the recharge progress.
    #[inline]
    pub fn recharge_progress(&self, passed: u32) -> f32 {
        self.recharge_remaining(passed) as f32 / self.recharge as f32
    }

    /// Returns the remaining ammo recharge.
    #[inline]
    pub fn ammo_recharge_remaining(&self, passed: u32) -> u32 {
        self.ammo_recharge_remaining.saturating_sub(passed)
    }

    /// Returns the ammo recharge progress.
    #[inline]
    pub fn ammo_recharge_progress(&self, passed: u32) -> f32 {
        self.ammo_recharge_remaining(passed) as f32 / self.ammo_recharge as f32
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
)]
pub enum Slot {
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

/// Recharge and timestmap.
#[derive(Debug, Default, Clone)]
pub struct Recharge {
    /// Last update timestamp.
    pub last_update: u32,

    /// Recharge in milliseconds.
    pub recharge: u32,
}

impl Recharge {
    /// Returns the remaining recharge.
    #[inline]
    pub fn recharge_remaining(&self, now: u32) -> u32 {
        (self.last_update + self.recharge).saturating_sub(now)
    }
}
