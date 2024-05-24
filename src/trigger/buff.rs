use super::{BuffThreshold, BuffTriggerId, Trigger};
use crate::{context::RenderContext, elements::RenderState, traits::RenderOptions};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

// TODO: we are still checking threshold for always/none

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BuffTrigger {
    pub id: BuffTriggerId,

    #[serde(rename = "stacks")]
    #[serde(alias = "threshold")]
    pub threshold: BuffThreshold,
}

impl Trigger for BuffTrigger {
    fn is_active(&mut self, ctx: &RenderContext) -> bool {
        self.threshold.is_met(self.id.count_stacks(ctx))
    }
}

impl BuffTrigger {
    pub fn active_stacks_or_edit(&self, ctx: &RenderContext, state: &RenderState) -> Option<i32> {
        if state.edit {
            Some(1)
        } else {
            let stacks = self.id.count_stacks(ctx);
            self.threshold.is_met(stacks).then_some(stacks)
        }
    }
}

impl RenderOptions for BuffTrigger {
    fn render_options(&mut self, ui: &Ui) {
        self.id.render_options(ui);
        self.threshold.render_options(ui);
    }
}
