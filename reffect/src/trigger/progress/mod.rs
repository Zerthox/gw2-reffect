mod active;
mod info;
mod source;
mod threshold;

pub use self::{active::*, info::*, source::*, threshold::*};

use crate::{
    context::{Context, ContextUpdate},
    render::{debug_optional, RenderDebug, RenderOptions},
    serde::migrate,
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
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

impl ProgressTrigger {
    pub fn buff() -> Self {
        Self {
            source: ProgressSource::Buff(vec![0]),
            threshold: ProgressThreshold {
                threshold_type: ThresholdType::Present,
                amount_type: AmountType::default(),
            },
            active_memo: None,
        }
    }

    pub fn update(&mut self, ctx: &Context, edited: bool, parent: Option<&ProgressActive>) {
        if ctx.has_update_or_edit(ContextUpdate::State) {
            // TODO: end of edit causes memo to "flash", maybe flag to end edit mode?
            self.active_memo = self.active_updated(ctx, edited, parent);
        }
    }

    fn active_updated(
        &mut self,
        ctx: &Context,
        edited: bool,
        parent: Option<&ProgressActive>,
    ) -> Option<ProgressActive> {
        if ctx.edit.is_editing() {
            if edited {
                Some(self.source.progress_edit(ctx, parent))
            } else {
                None
            }
        } else {
            self.source
                .progress(ctx, parent)
                .filter(|active| self.source.no_threshold() || self.threshold.is_met(active, ctx))
        }
    }

    pub fn active(&self) -> Option<&ProgressActive> {
        self.active_memo.as_ref()
    }
}

impl RenderOptions for ProgressTrigger {
    fn render_options(&mut self, ui: &Ui, ctx: &Context) {
        let _id = ui.push_id("trigger");
        self.source.render_options(ui, ctx);
        if !self.source.no_threshold() {
            self.threshold.render_options(ui, ctx);
            // TODO: we rely on interval refreshing the memo, render options might want context for updates
        }
    }
}

impl RenderDebug for ProgressTrigger {
    fn render_debug(&mut self, ui: &Ui, _ctx: &Context) {
        debug_optional(ui, "Trigger", self.active());
    }
}

impl Clone for ProgressTrigger {
    fn clone(&self) -> Self {
        Self {
            source: self.source.clone(),
            threshold: self.threshold.clone(),
            active_memo: None, // dont clone the memo
        }
    }
}
