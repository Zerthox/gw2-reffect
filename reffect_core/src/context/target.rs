use crate::{
    context::{Affinity, CombatantResources, skill::BuffMap},
    error::Error,
};

/// Target info.
#[derive(Debug, Clone)]
pub struct TargetInfo {
    /// Affinity.
    pub affinity: Result<Affinity, Error>,

    /// Generic combatant resources.
    pub resources: Result<CombatantResources, Error>,

    /// Target buffs.
    pub buffs: Result<BuffMap, Error>,
}

impl TargetInfo {
    #[inline]
    pub const fn disabled() -> Self {
        Self {
            affinity: Err(Error::Disabled),
            resources: Err(Error::Disabled),
            buffs: Err(Error::Disabled),
        }
    }

    #[inline]
    pub const fn empty() -> Self {
        Self {
            affinity: Ok(Affinity::Hostile),
            resources: Ok(CombatantResources::empty()),
            buffs: Ok(BuffMap::new()),
        }
    }

    #[inline]
    pub fn set_error(&mut self, error: Error) {
        self.affinity = Err(error.clone());
        self.resources = Err(error.clone());
        self.buffs = Err(error);
    }
}
