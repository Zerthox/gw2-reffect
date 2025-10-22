use crate::{
    context::{BuffMap, CombatantResources},
    error::Error,
};

/// Group member.
#[derive(Debug, Clone)]
pub struct GroupMember {
    /// Group member account name.
    pub account: Option<String>,

    /// Generic combatant resources.
    pub resources: Result<CombatantResources, Error>,

    /// Group member buffs.
    pub buffs: Result<BuffMap, Error>,
}

impl GroupMember {
    #[inline]
    pub const fn empty() -> Self {
        Self {
            account: None,
            resources: Err(Error::Disabled),
            buffs: Err(Error::Disabled),
        }
    }
}

impl Default for GroupMember {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
