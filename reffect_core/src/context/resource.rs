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
pub enum ResourceType {
    Health = 1 << 0,

    Barrier = 1 << 1,

    Profession = 1 << 2,

    Endurance = 1 << 3,

    #[strum(serialize = "Defiance Immune")]
    DefianceImmune = 1 << 4,

    #[strum(serialize = "Defiance Active")]
    DefianceActive = 1 << 5,

    #[strum(serialize = "Defiance Recover")]
    DefianceRecover = 1 << 6,
}

impl Named for ResourceType {
    #[inline]
    fn name(&self) -> &'static str {
        self.into()
    }

    #[inline]
    fn short_name(&self) -> &'static str {
        match self {
            Self::Health => "Hp",
            Self::Barrier => "Bar",
            Self::Profession => "Prof",
            Self::Endurance => "End",
            Self::DefianceImmune => "Imm",
            Self::DefianceActive => "Break",
            Self::DefianceRecover => "Rec",
        }
    }
}

impl Colored for ResourceType {
    #[inline]
    fn colored(&self) -> Option<Color> {
        match self {
            Self::Health => Some(colors::RED),
            Self::Barrier => Some(colors::YELLOW),
            Self::Profession => Some(colors::BLUE),
            Self::Endurance => Some(colors::ORANGE),
            Self::DefianceImmune => Some(colors::LIGHT_GREY),
            Self::DefianceActive => Some(colors::CYAN),
            Self::DefianceRecover => Some(colors::ORANGE),
        }
    }
}
