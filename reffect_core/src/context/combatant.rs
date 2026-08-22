use crate::context::{Resource, ResourceState};

/// Generic combatant resources.
#[derive(Debug, Clone)]
pub struct CombatantResources {
    /// Health state.
    pub health_state: ResourceState,

    /// Health.
    pub health: Resource,

    /// Barrier.
    pub barrier: Resource,

    /// Defiance.
    pub defiance: Option<Defiance>,
}

impl CombatantResources {
    /// Creates empty combatant resources.
    #[inline]
    pub const fn empty() -> Self {
        Self {
            health_state: ResourceState::HealthAlive,
            health: Resource::empty(),
            barrier: Resource::empty(),
            defiance: None,
        }
    }

    /// Creates new combatant resources.
    #[inline]
    pub const fn new(
        normalize: bool,
        health_state: ResourceState,
        current_health: f32,
        current_barrier: f32,
        max_health: f32,
        defiance: Option<Defiance>,
    ) -> Self {
        if !normalize {
            Self {
                health_state,
                health: Resource::new(current_health, max_health),
                barrier: Resource::new(current_barrier, max_health),
                defiance,
            }
        } else {
            Self {
                health_state,
                health: Resource::new(100.0 * current_health / max_health, 100.0),
                barrier: Resource::new(100.0 * current_barrier / max_health, 100.0),
                defiance,
            }
        }
    }
}

impl Default for CombatantResources {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Defiance {
    Immune,
    Active(f32),
    Recover(f32),
}

impl Defiance {
    #[inline]
    pub const fn percent(&self) -> f32 {
        match *self {
            Self::Immune => 100.0,
            Self::Active(percent) | Self::Recover(percent) => percent,
        }
    }

    #[inline]
    pub const fn state(&self) -> ResourceState {
        match *self {
            Self::Immune => ResourceState::DefianceImmune,
            Self::Active(_) => ResourceState::DefianceActive,
            Self::Recover(_) => ResourceState::DefianceRecover,
        }
    }
}
