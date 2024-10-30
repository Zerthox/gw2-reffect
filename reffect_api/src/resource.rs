/// Information about resources.
#[derive(Debug, Clone)]
pub struct Resources {
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

impl Resources {
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

impl Default for Resources {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

/// Information about a resource.
#[derive(Debug, Clone)]
pub struct Resource {
    /// Current amount.
    pub current: u32, // TODO: as f32?

    /// Maximum amount.
    pub max: u32,
}

impl Resource {
    /// Creates an empty resource.
    #[inline]
    pub const fn empty() -> Self {
        Self::new(0, 0)
    }

    /// Creates a resources with the given values.
    #[inline]
    pub const fn new(current: u32, max: u32) -> Self {
        Self { current, max }
    }

    /// Creates a resource from interpolated values (ceiling).
    #[inline]
    pub fn interpolated_ceil(amount: f32, max: f32, gain: f32, passed: u32) -> Resource {
        let passed = passed as f32 / 1000.0;
        let current = amount + passed * gain;
        Resource::new(
            if current < 0.0 {
                0
            } else if current > max {
                max.ceil() as u32
            } else {
                current.ceil() as u32
            },
            max.ceil() as u32,
        )
    }

    /// Creates a resource from interpolated values (flooring).
    #[inline]
    pub fn interpolated_floor(amount: f32, max: f32, gain: f32, passed: u32) -> Resource {
        let passed = passed as f32 / 1000.0;
        let current = amount + passed * gain;
        Resource::new(
            if current < 0.0 {
                0
            } else if current > max {
                max.floor() as u32
            } else {
                current.floor() as u32
            },
            max.floor() as u32,
        )
    }
}

impl Default for Resource {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
