use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{AsRefStr, Display, EnumCount, EnumIter, IntoStaticStr, VariantArray};

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
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let Self { current, max } = self;
        current.fmt(formatter)?;
        formatter.write_str("/")?;
        max.fmt(formatter)
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Display,
    AsRefStr,
    IntoStaticStr,
    EnumCount,
    EnumIter,
    VariantArray,
    Serialize,
    Deserialize,
)]
pub enum ResourceType {
    Generic,

    Health,
    Barrier,
    Profession,
    Endurance,

    #[strum(serialize = "Defiance Immune")]
    DefianceImmune,

    #[strum(serialize = "Defiance Active")]
    DefianceActive,

    #[strum(serialize = "Defiance Recover")]
    DefianceRecover,
}

impl ResourceType {
    pub const DEFAULT: Self = Self::Generic;
}

impl Default for ResourceType {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}
