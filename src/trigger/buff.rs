use super::{BuffThreshold, BuffTriggerId, Trigger};
use crate::{
    context::{Context, ContextUpdate},
    elements::RenderState,
    traits::RenderOptions,
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

// TODO: update examples!

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BuffTrigger {
    /// Buff id(s).
    pub id: BuffTriggerId,

    /// Buff threshold requirement.
    #[serde(alias = "stacks")]
    pub threshold: BuffThreshold,

    /// Memoized active stacks.
    #[serde(skip)]
    active_memo: Option<i32>,
}

impl Trigger for BuffTrigger {
    fn is_active(&mut self, ctx: &Context) -> bool {
        self.active_stacks(ctx).is_some()
    }
}

impl BuffTrigger {
    fn force_update(&mut self, ctx: &Context) {
        let stacks = self.id.count_stacks(ctx);
        self.active_memo = (self.id.always() || self.threshold.is_met(stacks)).then_some(stacks);
    }

    pub fn active_stacks(&mut self, ctx: &Context) -> Option<i32> {
        if ctx.has_update_or_edit(ContextUpdate::Buffs) {
            self.force_update(ctx);
        }
        self.active_memo
    }

    pub fn active_stacks_or_edit(&mut self, ctx: &Context, state: &RenderState) -> Option<i32> {
        if state.edit {
            Some(1)
        } else {
            self.active_stacks(ctx)
        }
    }
}

impl RenderOptions for BuffTrigger {
    fn render_options(&mut self, ui: &Ui) {
        self.id.render_options(ui);
        if !self.id.always() {
            self.threshold.render_options(ui);
            // TODO: we rely on buffs interval refreshing the memo
        }
    }
}
