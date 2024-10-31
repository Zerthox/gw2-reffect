use super::{Buff, Error, Resources};

/// Current state.
#[derive(Debug, Clone)]
pub struct State {
    /// Own buffs.
    pub own_buffs: Result<Vec<Buff>, Error>,

    /// Own resources.
    pub own_resources: Result<Resources, Error>,

    /// Target buffs.
    pub target_buffs: Result<Vec<Buff>, Error>,
}

impl State {
    #[inline]
    pub const fn disabled() -> Self {
        Self {
            own_buffs: Err(Error::Disabled),
            own_resources: Err(Error::Disabled),
            target_buffs: Err(Error::Disabled),
        }
    }

    #[inline]
    pub fn with_err(err: Error) -> Self {
        Self {
            own_buffs: Err(err.clone()),
            own_resources: Err(err.clone()),
            target_buffs: Err(err),
        }
    }

    #[inline]
    pub fn set_error(&mut self, err: Error) {
        *self = Self::with_err(err);
    }
}
