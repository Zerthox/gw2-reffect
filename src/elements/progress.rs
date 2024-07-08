use crate::{context::Context, trigger::ActiveBuff};
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
    pub fn calc(&self, ctx: &Context, active: &ActiveBuff, max: u32) -> f32 {
        match self {
            Self::Duration => ctx
                .progress_remaining(active.apply, active.runout)
                .unwrap_or(1.0), // default to full bar
            Self::Intensity => active.stacks as f32 / max as f32,
        }
    }
}
