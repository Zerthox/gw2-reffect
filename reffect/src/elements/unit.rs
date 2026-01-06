use const_default::ConstDefault;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, VariantArray};

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
    EnumIter,
    VariantArray,
    Serialize,
    Deserialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum Unit {
    Percent,
    Absolute,
}

impl ConstDefault for Unit {
    const DEFAULT: Self = Self::Percent;
}

impl Default for Unit {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Unit {
    pub fn calc_progress(&self, value: f32, max: f32) -> Option<f32> {
        match self {
            Self::Percent => Some(value),
            Self::Absolute => {
                if max == 0.0 {
                    None
                } else {
                    Some(value / max)
                }
            }
        }
    }
}
