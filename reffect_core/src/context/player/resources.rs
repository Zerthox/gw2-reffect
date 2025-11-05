use crate::context::{CombatantResources, resource::Resource};

/// Information about player resources.
#[derive(Debug, Clone)]
pub struct PlayerResources {
    /// Generic combatant resources.
    pub combatant: CombatantResources,

    /// Health reduction.
    pub health_reduction: Resource,

    /// Pet resources.
    pub pet: Option<CombatantResources>,

    /// Endurance.
    pub endurance: Resource,

    /// Primary profession resource.
    // TODO: separate error state for profession resources?
    pub primary: Resource,

    /// Secondary profession resource.
    pub secondary: Resource,
}

impl PlayerResources {
    /// Creates empty resources.
    #[inline]
    pub const fn empty() -> Self {
        Self {
            combatant: CombatantResources::empty(),
            health_reduction: Resource::empty(),
            pet: None,
            endurance: Resource::empty(),
            primary: Resource::empty(),
            secondary: Resource::empty(),
        }
    }
}

impl Default for PlayerResources {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
