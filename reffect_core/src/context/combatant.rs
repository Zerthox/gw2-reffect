use crate::context::Resource;

/// Generic combatant resources.
#[derive(Debug, Clone)]
pub struct CombatantResources {
    /// Whether the resources are normalized.
    pub normalized: bool,

    /// Health.
    pub health: Resource,

    /// Barrier.
    pub barrier: Resource,

    /// Defiance.
    pub defiance: Defiance,
}

impl CombatantResources {
    /// Creates empty combatant resources.
    #[inline]
    pub const fn empty(normalized: bool) -> Self {
        Self {
            normalized,
            health: Resource::empty(),
            barrier: Resource::empty(),
            defiance: Defiance::DEFAULT,
        }
    }

    /// Creates new combatant resources.
    #[inline]
    pub const fn new(
        normalize: bool,
        current_health: f32,
        current_barrier: f32,
        max_health: f32,
        defiance: Defiance,
    ) -> Self {
        if !normalize {
            Self {
                normalized: false,
                health: Resource::new(current_health, max_health),
                barrier: Resource::new(current_barrier, max_health),
                defiance,
            }
        } else {
            Self {
                normalized: true,
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
        Self::empty(true)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Defiance {
    #[default]
    None,
    Immune,
    Active(f32),
    Recover(f32),
}

impl Defiance {
    pub const DEFAULT: Self = Self::None;

    #[inline]
    pub fn percent(&self) -> Option<f32> {
        match *self {
            Self::None => None,
            Self::Immune => Some(100.0),
            Self::Active(percent) | Self::Recover(percent) => Some(percent),
        }
    }
}
