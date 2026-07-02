use crate::{
    context::Context,
    elements::text::fragment::TextFragment,
    trigger::{ProgressSource, ProgressTrigger},
};
use strum::{AsRefStr, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, AsRefStr, Display)]
pub enum Processing {
    /// Static text, no processing.
    Static,

    /// Reprocess on trigger update.
    Trigger,

    /// Reprocess each frame.
    Frame,
}

impl Processing {
    pub const MIN: Self = Self::Static;

    pub fn or(&mut self, other: Self) {
        *self = (*self).max(other);
    }

    pub fn resolve(fragment: &TextFragment<'_>, source: &ProgressSource) -> Self {
        match fragment {
            TextFragment::Literal(_) | TextFragment::Name => Self::Static,
            TextFragment::Intensity { .. } => Self::Trigger,
            TextFragment::Current { .. }
            | TextFragment::Full { .. }
            | TextFragment::Percent { .. } => {
                if source.is_timed() {
                    Self::Frame
                } else {
                    Self::Trigger
                }
            }
        }
    }

    pub fn needs_reprocess(&self, ctx: &Context, trigger: &ProgressTrigger) -> bool {
        match self {
            Self::Static => false,
            Self::Trigger => ctx.edit.is_editing() || trigger.needs_update(ctx, None),
            Self::Frame => true,
        }
    }
}
