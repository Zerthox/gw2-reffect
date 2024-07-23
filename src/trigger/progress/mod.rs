mod active;
mod source;
mod threshold;

pub use self::{active::*, source::*, threshold::*};

use super::Trigger;
use crate::{
    context::{Context, ContextUpdate},
    elements::RenderState,
    traits::RenderOptions,
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ProgressTrigger {
    /// Progress source.
    #[serde(alias = "id")] // TODO: remove backwards compat
    pub source: ProgressSource,

    /// Threshold requirement.
    #[serde(alias = "stacks")]
    pub threshold: ProgressThreshold,

    /// Memoized active progress.
    #[serde(skip)]
    active_memo: Option<ProgressActive>,
}

impl Trigger for ProgressTrigger {
    fn is_active(&mut self, ctx: &Context) -> bool {
        self.active(ctx).is_some()
    }
}

impl ProgressTrigger {
    fn force_update(&mut self, ctx: &Context) {
        self.active_memo = self
            .source
            .progress(ctx)
            .filter(|active| self.source.always() || self.threshold.is_met(active.intensity()));
    }

    pub fn active(&mut self, ctx: &Context) -> Option<&ProgressActive> {
        if ctx.has_update_or_edit(ContextUpdate::OwnCharacter) {
            self.force_update(ctx);
        }
        self.active_memo.as_ref()
    }

    pub fn active_or_edit(&mut self, ctx: &Context, state: &RenderState) -> Option<ProgressActive> {
        if ctx.edit.is_editing() {
            if state.is_edit(ctx) {
                Some(self.source.progress_edit(ctx))
            } else {
                None
            }
        } else {
            self.active(ctx).cloned()
        }
    }
}

impl RenderOptions for ProgressTrigger {
    fn render_options(&mut self, ui: &Ui) {
        self.source.render_options(ui);
        if !self.source.always() {
            self.threshold.render_options(ui);
            // TODO: we rely on interval refreshing the memo, render options might want context for updates
        }
    }
}
