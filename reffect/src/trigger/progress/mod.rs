mod active;
mod source;
mod threshold;
mod value;

pub use self::{active::*, source::*, threshold::*, value::*};

use crate::{
    context::{Context, Update},
    render::debug_optional,
    serde::migrate,
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct ProgressTrigger {
    /// Progress source.
    #[serde(alias = "id")] // TODO: remove backwards compat
    #[serde(deserialize_with = "migrate::<_, _, ProgressSourceLegacy>")]
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
            source: ProgressSource::Buff {
                combatant: Combatant::default(),
                ids: vec![0],
            },
            threshold: ProgressThreshold {
                threshold_type: ThresholdType::Present,
                amount_type: AmountType::default(),
            },
            active_memo: None,
        }
    }

    pub fn update(&mut self, ctx: &Context, parent: Option<&ProgressActive>) -> bool {
        if ctx.has_update_or_edit(Update::Game) {
            // TODO: end of edit causes memo to "flash", maybe flag to end edit mode?
            self.active_memo = self.active_updated(ctx, parent);
        }
        self.active_memo.is_some()
    }

    fn active_updated(
        &mut self,
        ctx: &Context,
        parent: Option<&ProgressActive>,
    ) -> Option<ProgressActive> {
        if ctx.edit.is_editing() {
            Some(self.source.progress_edit(ctx, parent))
        } else {
            self.source
                .progress(ctx, parent)
                .filter(|active| self.source.no_threshold() || self.threshold.is_met(active, ctx))
        }
    }

    pub fn active(&self) -> Option<&ProgressActive> {
        self.active_memo.as_ref()
    }

    pub fn render_options(&mut self, ui: &Ui, ctx: &Context) {
        let mut changed = false;
        let _id = ui.push_id("trigger");

        changed |= self.source.render_options(ui);

        if !self.source.no_threshold() {
            changed |= self.threshold.render_options(ui);
        }

        if changed {
            self.update(ctx, None);
        }
    }

    pub fn render_debug(&mut self, ui: &Ui) {
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
