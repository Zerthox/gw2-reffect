mod active;
mod source;
mod threshold;

pub use self::{active::*, source::*, threshold::*};

use super::Trigger;
use crate::{
    context::{Context, ContextUpdate, EditState},
    elements::RenderState,
    serde_migrate::migrate,
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
    #[serde(deserialize_with = "migrate::<_, _, ThresholdType>")]
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
            .filter(|active| self.source.always() || self.threshold.is_met(active, ctx));
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
    fn render_options(&mut self, ui: &Ui, state: &mut EditState) {
        self.source.render_options(ui, state);
        if !self.source.always() {
            self.threshold.render_options(ui, state);
            // TODO: we rely on interval refreshing the memo, render options might want context for updates
        }
    }
}
