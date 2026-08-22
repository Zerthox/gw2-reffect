use enumflags2::bitflags;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{AsRefStr, Display, EnumIter, IntoStaticStr, VariantArray};

use crate::{
    colors::{self, Color, Colored},
    named::Named,
};

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
    AsRefStr,
    IntoStaticStr,
    Display,
    EnumIter,
    VariantArray,
    Serialize,
    Deserialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(u8)]
#[bitflags]
pub enum ResourceState {
    #[strum(serialize = "Health Alive")]
    HealthAlive = 1 << 0,

    #[strum(serialize = "Health Downed")]
    HealthDowned = 1 << 1,

    #[strum(serialize = "Health Dead")]
    HealthDead = 1 << 2,

    #[strum(serialize = "Defiance Immune")]
    DefianceImmune = 1 << 3,

    #[strum(serialize = "Defiance Active")]
    DefianceActive = 1 << 4,

    #[strum(serialize = "Defiance Recover")]
    DefianceRecover = 1 << 5,
}

impl Named for ResourceState {
    #[inline]
    fn name(&self) -> &'static str {
        self.into()
    }

    #[inline]
    fn short_name(&self) -> &'static str {
        match self {
            Self::HealthAlive => "Alive",
            Self::HealthDowned => "Down",
            Self::HealthDead => "Dead",
            Self::DefianceImmune => "Immune",
            Self::DefianceActive => "Active",
            Self::DefianceRecover => "Recover",
        }
    }
}

impl Colored for ResourceState {
    #[inline]
    fn colored(&self) -> Option<Color> {
        match self {
            Self::HealthAlive | Self::HealthDowned | Self::HealthDead => Some(colors::RED),
            Self::DefianceImmune | Self::DefianceActive | Self::DefianceRecover => {
                Some(colors::CYAN)
            }
        }
    }
}
