use crate::context::Resource;

/// Generic combatant resources.
#[derive(Debug, Clone)]
pub struct CombatantResources {
    /// Health.
    pub health: Resource,

    /// Barrier.
    pub barrier: Resource,

    /// Defiance.
    pub defiance: Defiance,
}

impl CombatantResources {
    /// Creates empty resources.
    #[inline]
    pub const fn empty() -> Self {
        Self {
            health: Resource::empty(),
            barrier: Resource::empty(),
            defiance: Defiance::None,
        }
    }
}

impl Default for CombatantResources {
    #[inline]
    fn default() -> Self {
        Self::empty()
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
    #[inline]
    pub fn percent(&self) -> Option<f32> {
        match *self {
            Self::None => None,
            Self::Immune => Some(100.0),
            Self::Active(percent) | Self::Recover(percent) => Some(percent),
        }
    }
}
