use crate::{context::BuffMap, error::Error};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, IntoStaticStr, VariantArray};

/// Group info.
#[derive(Debug, Clone)]
pub struct GroupInfo {
    /// Group type.
    pub group_type: GroupType,

    /// Group members.
    pub members: GroupMembers,
}

impl GroupInfo {
    /// Maximum number of displayed members.
    pub const MAX_MEMBERS: usize = 4;

    /// Creates an empty group.
    #[inline]
    pub const fn empty() -> Self {
        Self {
            group_type: GroupType::Solo,
            members: [const { None }; Self::MAX_MEMBERS],
        }
    }
}

/// Group Members.
pub type GroupMembers = [Option<GroupMember>; GroupInfo::MAX_MEMBERS];

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
    pub account: String,

    /// Group member health.
    pub resources: Result<GroupMemberResources, Error>,

    /// Group member buffs.
    pub buffs: Result<BuffMap, Error>,
}

impl GroupMember {
    #[inline]
    pub const fn new() -> Self {
        Self {
            account: String::new(),
            resources: Err(Error::Disabled),
            buffs: Err(Error::Disabled),
        }
    }
}

impl Default for GroupMember {
    #[inline]
    fn default() -> Self {
        Self::new()
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
