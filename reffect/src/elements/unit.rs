use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, VariantArray};

#[derive(
    Debug,
    Default,
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
pub enum Unit {
    #[default]
    Percent,
    Absolute,
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
