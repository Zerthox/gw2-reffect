use crate::{BuffMap, Error, Resources, Result, Skillbar};

/// Current state.
#[derive(Debug, Clone)]
pub struct State {
    /// Own resources.
    pub own_resources: Result<Resources>,

    /// Own skillbar.
    pub own_skillbar: Result<Skillbar>,

    /// Own buffs.
    pub own_buffs: Result<BuffMap>,

    /// Target buffs.
    pub target_buffs: Result<BuffMap>,

    /// Group buffs.
    pub group_buffs: Result<[Option<BuffMap>; 4]>,
}

impl State {
    #[inline]
    pub const fn disabled() -> Self {
        Self {
            own_skillbar: Err(Error::Disabled),
            own_resources: Err(Error::Disabled),
            own_buffs: Err(Error::Disabled),
            target_buffs: Err(Error::Disabled),
            group_buffs: Err(Error::Disabled),
        }
    }

    #[inline]
    pub fn with_err(err: Error) -> Self {
        Self {
            own_resources: Err(err.clone()),
            own_skillbar: Err(err.clone()),
            own_buffs: Err(err.clone()),
            target_buffs: Err(err.clone()),
            group_buffs: Err(err),
        }
    }

    #[inline]
    pub fn set_error(&mut self, err: Error) {
        *self = Self::with_err(err);
    }
}
