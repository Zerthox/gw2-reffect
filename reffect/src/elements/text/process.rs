use crate::{
    context::Context,
    elements::text::fragment::TextFragment,
    trigger::{ProgressActive, ProgressTrigger},
};
use strum::{AsRefStr, Display};

/// Text processing strategy.
///
/// Ordering is important and corresponds to complexity/cost.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, AsRefStr, Display)]
pub enum Processing {
    /// Static text, no processing.
    Static = 0,

    /// Reprocess on trigger update.
    Trigger = 1,

    /// Reprocess each frame.
    Frame = 2,
}

impl Processing {
    /// Minimum trategy.
    pub const MIN: Self = Self::Static;

    /// Selects the higher strategy.
    pub fn or(&mut self, other: Self) {
        *self = (*self).max(other);
    }

    /// Resolves the strategy required to process the text fragment.
    pub fn resolve(fragment: &TextFragment, active: &ProgressActive) -> Self {
        match fragment {
            TextFragment::Literal(_) | TextFragment::Name => Self::Static,
            TextFragment::Intensity { .. } => Self::Trigger,
            TextFragment::Current { .. }
            | TextFragment::Full { .. }
            | TextFragment::Percent { .. } => {
                if active.is_timed() {
                    Self::Frame
                } else {
                    Self::Trigger
                }
            }
        }
    }

    /// Checks whether reprocessing is needed.
    pub fn needs_reprocess(&self, ctx: &Context, trigger: &ProgressTrigger) -> bool {
        match self {
            Self::Static => false,
            Self::Trigger => ctx.edit.is_editing() || trigger.needs_update(ctx, None),
            Self::Frame => true,
        }
    }
}
