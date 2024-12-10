use strum::{EnumCount, EnumIter};

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, EnumCount, EnumIter)]
pub enum Slot {
    Weapon1,
    Weapon2,
    Weapon3,
    Weapon4,
    Weapon5,
    Heal,
    Utility1,
    Utility2,
    Utility3,
    Elite,
    Profession1,
    Profession2,
    Profession3,
    Profession4,
    Profession5,
    SpecialAction,
    Mount,
}
