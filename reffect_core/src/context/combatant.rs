use crate::context::Resource;

/// Generic combatant resources.
#[derive(Debug, Clone)]
pub struct CombatantResources {
    /// Health.
    pub health: Resource,

    /// Barrier.
    pub barrier: Resource,

    /// Defiance.
    pub defiance: Option<f32>,
}

impl CombatantResources {
    /// Creates empty resources.
    #[inline]
    pub const fn empty() -> Self {
        Self {
            health: Resource::empty(),
            barrier: Resource::empty(),
            defiance: None,
        }
    }
}

impl Default for CombatantResources {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
