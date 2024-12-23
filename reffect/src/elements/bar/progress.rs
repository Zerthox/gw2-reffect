use crate::{
    context::Context,
    trigger::{ProgressActive, ProgressValue},
};
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
    Intensity,

    #[default]
    Duration,

    #[strum(serialize = "Secondary Duration")]
    SecondaryDuration,
}

impl Progress {
    pub fn calc_progress(&self, ctx: &Context, active: &ProgressActive, max: u32) -> f32 {
        match self {
            Self::Intensity => {
                if max > 0 {
                    active.intensity() as f32 / max as f32
                } else {
                    0.0
                }
            }
            Self::Duration => active.progress_or_default(ProgressValue::Primary, ctx.now),
            Self::SecondaryDuration => {
                active.progress_or_default(ProgressValue::Secondary, ctx.now)
            }
        }
    }

    pub fn progress_max(&self, active: &ProgressActive, max: u32) -> u32 {
        match self {
            Self::Intensity => max,
            Self::Duration => active.max(ProgressValue::Primary),
            Self::SecondaryDuration => active.max(ProgressValue::Secondary),
        }
    }
}
