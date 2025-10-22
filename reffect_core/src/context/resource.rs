use std::fmt;

/// Information about a resource.
#[derive(Debug, Clone)]
pub struct Resource {
    /// Current amount.
    pub current: f32,

    /// Maximum amount.
    pub max: f32,
}

impl Resource {
    /// Creates an empty resource.
    #[inline]
    pub const fn empty() -> Self {
        Self::new(0.0, 0.0)
    }

    /// Creates a resources with the given values.
    #[inline]
    pub const fn new(current: f32, max: f32) -> Self {
        Self { current, max }
    }
}

impl Default for Resource {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

impl fmt::Display for Resource {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self { current, max } = self;
        write!(f, "{current}/{max}")
    }
}
