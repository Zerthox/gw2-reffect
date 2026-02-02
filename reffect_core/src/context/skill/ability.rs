use super::SkillId;
use crate::{
    colors::{Color, Colored},
    named::Named,
};
use enumflags2::{BitFlags, bitflags};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, IntoStaticStr, VariantArray};

/// Ability.
#[derive(Debug, Clone)]
pub struct Ability {
    /// Ability identifier.
    pub id: SkillId,

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

    /// Ability state information.
    pub state: BitFlags<AbilityState>,
}

impl Ability {
    /// Creates a new ability without cooldowns.
    #[inline]
    pub fn new(id: impl Into<SkillId>, ammo: u32, last_update: u32, recharge_rate: f32) -> Self {
        Self {
            id: id.into(),
            ammo,
            last_update,
            recharge_rate,
            recharge: 0,
            recharge_remaining: 0,
            ammo_recharge: 0,
            ammo_recharge_remaining: 0,
            state: BitFlags::empty(),
        }
    }

    /// Creates a new ability with simple recharge.
    #[inline]
    pub fn simple(id: impl Into<SkillId>, last_update: u32, recharge: u32) -> Self {
        Self {
            id: id.into(),
            ammo: if recharge == 0 { 1 } else { 0 },
            last_update,
            recharge_rate: 1.0,
            recharge,
            recharge_remaining: recharge,
            ammo_recharge: 0,
            ammo_recharge_remaining: 0,
            state: BitFlags::empty(),
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

    /// Returns the end time for the recharge.
    #[inline]
    pub fn recharge_end(&self) -> u32 {
        self.last_update + (self.recharge_remaining as f32 / self.recharge_rate) as u32
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
#[repr(u8)]
#[bitflags]
pub enum AbilityState {
    /// Ability is auto-attack.
    #[strum(serialize = "Auto Attack")]
    AutoAttack = 1 << 0,

    /// Ability is queued or casting.
    Pending = 1 << 1,

    /// Ability is pressed.
    Pressed = 1 << 2,

    /// Ability is active primary.
    #[strum(serialize = "Active Primary")]
    ActivePrimary = 1 << 3,

    /// Ability is active secondary.
    #[strum(serialize = "Active Secondary")]
    ActiveSecondary = 1 << 4,

    // Missing resource for ability.
    #[strum(serialize = "No Resources")]
    NoResources = 1 << 5,

    // Not in range for ability.
    #[strum(serialize = "No Range")]
    NoRange = 1 << 6,
}

impl Named for AbilityState {
    fn name(&self) -> &'static str {
        self.into()
    }

    fn short_name(&self) -> &'static str {
        match self {
            Self::AutoAttack => "Auto",
            Self::Pressed => "Press",
            Self::Pending => "Pend",
            Self::ActivePrimary => "Act1",
            Self::ActiveSecondary => "Act2",
            Self::NoResources => "Res",
            Self::NoRange => "Range",
        }
    }
}

impl Colored for AbilityState {
    fn colored(&self) -> Option<Color> {
        None
    }
}
