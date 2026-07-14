mod active;
mod source;
mod threshold;
mod value;

pub use self::{active::*, source::*, threshold::*, value::*};

use crate::{context::Context, render::debug_optional, serde::migrate};
use const_default::ConstDefault;
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

/// A progress trigger.
#[derive(Debug, Default, ConstDefault, Serialize, Deserialize)]
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
    active: Option<ProgressActive>,
}

impl ProgressTrigger {
    /// Creates a new progress trigger with the given source.
    pub const fn with(source: ProgressSource) -> Self {
        Self {
            source,
            threshold: ProgressThreshold::DEFAULT,
            active: None,
        }
    }

    /// Returns the default buff progress trigger.
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
            active: None,
        }
    }

    /// Returns the current [`ProgressActive`] if present.
    pub fn active(&self) -> Option<&ProgressActive> {
        self.active.as_ref()
    }

    /// Checks whether the element is visible (trigger is active).
    pub fn is_visible(&self) -> bool {
        self.active.is_some()
    }

    /// Checks whether the trigger needs an update.
    pub fn needs_update(&self, ctx: &Context, parent: Option<&Self>) -> bool {
        let updates = if self.source.inherits()
            && let Some(parent) = parent
        {
            parent.source.update_on()
        } else {
            self.source.update_on()
        };
        ctx.has_update_or_edit(updates)
    }

    /// Updates the trigger if forced or needed.
    pub fn update(&mut self, ctx: &Context, parent: Option<&Self>, force: bool) {
        if force || self.needs_update(ctx, parent) {
            self.force_update(ctx, parent);
        }
    }

    /// Force updates the trigger.
    pub fn force_update(&mut self, ctx: &Context, parent: Option<&Self>) {
        // TODO: end of edit causes memo to "flash", maybe flag to end edit mode?
        self.active = self.resolve_active(ctx, parent.and_then(|trigger| trigger.active()));
    }

    /// Resolves the current [`ProgressActive`].
    fn resolve_active(
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

    /// Renders the trigger options.
    pub fn render_options(&mut self, ui: &Ui, ctx: &Context) {
        let mut changed = false;
        let _id = ui.push_id("trigger");

        changed |= self.source.render_options(ui);

        if !self.source.no_threshold() {
            changed |= self.threshold.render_options(ui);
        }

        if changed {
            self.force_update(ctx, None);
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
            active: None, // dont clone the memo
        }
    }
}
