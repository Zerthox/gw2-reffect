use crate::{
    context::{CombatantResources, skill::BuffMap},
    error::Error,
};

/// Target info.
#[derive(Debug, Clone)]
pub struct TargetInfo {
    /// Generic combatant resources.
    pub resources: Result<CombatantResources, Error>,

    /// Target buffs.
    pub buffs: Result<BuffMap, Error>,
}

impl TargetInfo {
    #[inline]
    pub const fn empty() -> Self {
        Self {
            resources: Err(Error::Disabled),
            buffs: Err(Error::Disabled),
        }
    }

    #[inline]
    pub fn set_error(&mut self, error: Error) {
        self.resources = Err(error.clone());
        self.buffs = Err(error);
    }
}
