use crate::{
    context::Context,
    trigger::{ProgressActive, ProgressValue},
};
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
pub enum Progress {
    Fixed,

    Intensity,

    Duration,

    #[strum(serialize = "Secondary Duration")]
    SecondaryDuration,
}

impl ConstDefault for Progress {
    const DEFAULT: Self = Self::Duration;
}

impl Default for Progress {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Progress {
    pub const fn use_max(&self) -> bool {
        matches!(self, Self::Intensity)
    }

    pub fn calc_progress(&self, ctx: &Context, active: &ProgressActive, max: f32) -> f32 {
        match self {
            Self::Fixed => 1.0,
            Self::Intensity => {
                if max > 0.0 {
                    active.intensity() as f32 / max
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

    pub fn progress_max(&self, active: &ProgressActive, max: f32) -> f32 {
        match self {
            Self::Fixed => 1.0,
            Self::Intensity => max,
            Self::Duration => active.max(ProgressValue::Primary),
            Self::SecondaryDuration => active.max(ProgressValue::Secondary),
        }
    }
}
