use crate::context::Context;
use enumflags2::{BitFlags, bitflags};

pub type Updates = BitFlags<Update>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[bitflags]
#[repr(u32)]
pub enum Update {
    /// Player identity update.
    PlayerIdentity = 1 << 0,

    /// Player resources update.
    PlayerResources = 1 << 1,

    /// Player buffs update.
    PlayerBuffs = 1 << 2,

    /// Player build update.
    PlayerBuild = 1 << 3,

    /// Player gear update.
    PlayerGear = 1 << 4,

    /// Player skillbar update.
    PlayerSkillbar = 1 << 5,

    /// Pet resources.
    PetResources = 1 << 6,

    /// Target identity update.
    TargetIdentity = 1 << 7,

    /// Target state update.
    TargetResources = 1 << 8,

    /// Target buffs update.
    TargetBuffs = 1 << 9,

    /// Group member identity update.
    GroupIdentity = 1 << 10,

    /// Group resources update.
    GroupResources = 1 << 11,

    /// Group buffs update.
    GroupBuffs = 1 << 12,

    /// Map update.
    Map = 1 << 13,
}

pub trait Updateable {
    /// Checks whether updates are needed.
    fn needs_update(&self, ctx: &Context) -> bool;

    /// Force updates the state.
    fn force_update(&mut self, ctx: &Context);

    /// Updates the state if needed.
    #[inline]
    fn update_if_need(&mut self, ctx: &Context) {
        if self.needs_update(ctx) {
            self.force_update(ctx);
        }
    }

    /// Updates the state if forced or needed.
    #[inline]
    fn update_if_forced_or_needed(&mut self, ctx: &Context, force: bool) {
        if force {
            self.force_update(ctx);
        } else {
            self.update_if_need(ctx);
        }
    }
}
