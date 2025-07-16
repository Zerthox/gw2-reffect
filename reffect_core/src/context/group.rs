use crate::{context::BuffMap, error::Error};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, IntoStaticStr, VariantArray};

/// Group info.
#[derive(Debug, Clone)]
pub struct GroupInfo {
    /// Group type.
    pub group_type: GroupType,

    /// Group members.
    pub members: [GroupMember; 4],
}

impl GroupInfo {
    /// Maximum number of displayed members.
    pub const MAX_MEMBERS: usize = 4;

    /// Creates an empty group.
    #[inline]
    pub const fn empty() -> Self {
        Self {
            group_type: GroupType::Solo,
            members: [const { GroupMember::empty() }; Self::MAX_MEMBERS],
        }
    }
}

impl Default for GroupInfo {
    fn default() -> Self {
        Self::empty()
    }
}

/// Group type.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    AsRefStr,
    IntoStaticStr,
    Display,
    EnumIter,
    VariantArray,
    Serialize,
    Deserialize,
)]
pub enum GroupType {
    Solo,
    Party,
    Squad,
}

/// Group member.
#[derive(Debug, Clone)]
pub struct GroupMember {
    /// Group member account name.
    pub account: Option<String>,

    /// Group member health.
    pub resources: Result<GroupMemberResources, Error>,

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

/// Group member resources.
#[derive(Debug, Clone)]
pub struct GroupMemberResources {
    /// Group member health.
    pub health: f32,

    /// Group member barrier.
    pub barrier: f32,
}
