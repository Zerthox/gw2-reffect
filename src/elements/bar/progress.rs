use crate::{context::Context, trigger::ProgressActive};
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
pub enum Progress {
    #[default]
    Duration,
    Intensity,
}

impl Progress {
    pub fn calc_progress(&self, ctx: &Context, active: &ProgressActive, max: u32) -> f32 {
        match self {
            Self::Duration => active.progress_or_default(ctx.now),
            Self::Intensity => {
                if max > 0 {
                    active.intensity() as f32 / max as f32
                } else {
                    0.0
                }
            }
        }
    }

    pub fn progress_max(&self, active: &ProgressActive, max: u32) -> u32 {
        match self {
            Self::Duration => active.max(),
            Self::Intensity => max,
        }
    }
}
