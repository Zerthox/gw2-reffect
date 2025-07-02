use crate::context::resource::Resource;

/// Information about player resources.
#[derive(Debug, Clone)]
pub struct PlayerResources {
    /// Health.
    pub health: Resource,

    /// Barrier.
    pub barrier: Resource,

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
            health: Resource::empty(),
            barrier: Resource::empty(),
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
