use crate::context::{CombatantResources, resource::Resource};

/// Information about player health resources.
#[derive(Debug, Clone)]
pub struct PlayerHealth {
    /// Generic combatant resources.
    pub combatant: CombatantResources,

    /// Health reduction.
    pub health_reduction: Resource,
}

impl PlayerHealth {
    /// Creates empty health.
    #[inline]
    pub const fn empty() -> Self {
        Self {
            combatant: CombatantResources::empty(),
            health_reduction: Resource::empty(),
        }
    }
}

impl Default for PlayerHealth {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

/// Information about player resources.
#[derive(Debug, Clone)]
pub struct PlayerResources {
    /// Pet resources.
    pub pet: Option<CombatantResources>,

    /// Endurance.
    pub endurance: Resource,

    /// Primary profession resource.
    // TODO: separate error state for profession resources?
    pub primary: Resource,

    /// Secondary profession resource.
    pub secondary: Resource,

    /// Profession resource rate.
    pub rate: Resource,
}

impl PlayerResources {
    /// Creates empty resources.
    #[inline]
    pub const fn empty() -> Self {
        Self {
            pet: None,
            endurance: Resource::empty(),
            primary: Resource::empty(),
            secondary: Resource::empty(),
            rate: Resource::empty(),
        }
    }
}

impl Default for PlayerResources {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
