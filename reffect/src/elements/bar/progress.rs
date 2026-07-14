use crate::{
    context::Context,
    trigger::{ProgressActive, ProgressValue},
};
use const_default::ConstDefault;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, VariantArray};

/// Progress type.
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
    /// Fixed, no progress.
    Fixed,

    /// Trigger intensity.
    Intensity,

    /// Trigger primary duration.
    Duration,

    /// Trigger secondary duration.
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
    /// Whether a max value is required.
    pub const fn use_max(&self) -> bool {
        matches!(self, Self::Intensity)
    }

    /// Calculates the progress value.
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

    /// Retrieves the maximum progress.
    pub fn progress_max(&self, active: &ProgressActive, max: f32) -> f32 {
        match self {
            Self::Fixed => 1.0,
            Self::Intensity => max,
            Self::Duration => active.max(ProgressValue::Primary),
            Self::SecondaryDuration => active.max(ProgressValue::Secondary),
        }
    }
}
