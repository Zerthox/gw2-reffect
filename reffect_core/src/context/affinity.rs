use crate::{
    colors::{self, Color, Colored},
    named::Named,
};
use enumflags2::bitflags;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, IntoStaticStr, VariantArray};

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
pub enum Affinity {
    Friendly = 1 << 0,

    Hostile = 1 << 1,

    Neutral = 1 << 2,
}

impl Named for Affinity {
    fn name(&self) -> &'static str {
        self.into()
    }
}

impl Colored for Affinity {
    fn colored(&self) -> Option<Color> {
        match self {
            Self::Friendly => Some(colors::GREEN),
            Self::Hostile => Some(colors::RED),
            Self::Neutral => Some(colors::YELLOW),
        }
    }
}
