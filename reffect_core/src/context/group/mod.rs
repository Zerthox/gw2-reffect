mod member;

pub use self::member::*;

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
