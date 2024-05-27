use super::{BuffThreshold, BuffTriggerId, Trigger};
use crate::{context::Context, elements::RenderState, traits::RenderOptions};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

// TODO: update examples!
// TODO: memoize?

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BuffTrigger {
    pub id: BuffTriggerId,

    #[serde(alias = "stacks")]
    pub threshold: BuffThreshold,
}

impl Trigger for BuffTrigger {
    fn is_active(&mut self, ctx: &Context) -> bool {
        self.id.always() || self.threshold.is_met(self.id.count_stacks(ctx))
    }
}

impl BuffTrigger {
    pub fn active_stacks_or_edit(&self, ctx: &Context, state: &RenderState) -> Option<i32> {
        if state.edit {
            Some(1)
        } else {
            let stacks = self.id.count_stacks(ctx);
            (self.id.always() || self.threshold.is_met(stacks)).then_some(stacks)
        }
    }
}

impl RenderOptions for BuffTrigger {
    fn render_options(&mut self, ui: &Ui) {
        self.id.render_options(ui);
        if !self.id.always() {
            self.threshold.render_options(ui);
        }
    }
}
