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
    /// Group member name.
    pub name: String,

    /// Group member buffs.
    pub buffs: Result<BuffMap, Error>,
}
