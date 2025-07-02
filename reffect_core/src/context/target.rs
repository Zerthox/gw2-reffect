use crate::{context::skill::BuffMap, error::Error};

/// Target info.
#[derive(Debug, Clone)]
pub struct TargetInfo {
    /// Target resources.
    pub resources: Result<TargetResources, Error>,

    /// Target buffs.
    pub buffs: Result<BuffMap, Error>,
}

impl TargetInfo {
    #[inline]
    pub const fn empty() -> Self {
        Self {
            resources: Err(Error::Disabled),
            buffs: Err(Error::Disabled),
        }
    }

    #[inline]
    pub fn set_err(&mut self, error: Error) {
        self.resources = Err(error.clone());
        self.buffs = Err(error.clone());
    }
}

/// Information about target resources.
#[derive(Debug, Clone)]
pub struct TargetResources {
    /// Health.
    pub health: f32,

    /// Barrier.
    pub barrier: f32,
}

impl TargetResources {
    /// Creates empty resources.
    #[inline]
    pub const fn empty() -> Self {
        Self {
            health: 0.0,
            barrier: 0.0,
        }
    }
}

impl Default for TargetResources {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
