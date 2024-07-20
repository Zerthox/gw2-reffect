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
    pub fn calc_progress(&self, value: f32, max: u32) -> Option<f32> {
        let progress = match self {
            Self::Percent => value,
            Self::Absolute => {
                if max == 0 {
                    return None;
                }
                value / max as f32
            }
        };
        (0.0 < progress && progress < 1.0).then_some(progress)
    }
}
