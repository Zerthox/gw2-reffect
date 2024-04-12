use super::Trigger;
use crate::{context::RenderContext, elements::RenderState};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum BuffTrigger {
    #[default]
    Always,

    Any(Vec<u32>),

    All(Vec<u32>),

    Not(u32),

    #[serde(untagged)]
    Single(u32),
}

impl Trigger for BuffTrigger {
    fn is_active(&self, ctx: &RenderContext) -> bool {
        match self {
            Self::Always => true,
            Self::Any(ids) => ids.iter().any(|id| ctx.has_buff(*id)),
            Self::All(ids) => ids.iter().any(|id| ctx.has_buff(*id)),
            Self::Not(id) => !ctx.has_buff(*id),
            Self::Single(id) => ctx.has_buff(*id),
        }
    }
}

impl BuffTrigger {
    fn get_stacks(&self, ctx: &RenderContext) -> Option<i32> {
        match self {
            Self::Always => Some(0),
            Self::Any(ids) => {
                let mut iter = ids.iter().filter_map(|id| ctx.stacks_of(*id));
                iter.next().map(|first| first + iter.sum::<i32>())
            }
            Self::All(ids) => {
                let mut sum = 0;
                for id in ids {
                    if let Some(stacks) = ctx.stacks_of(*id) {
                        sum += stacks;
                    } else {
                        return None;
                    }
                }
                Some(sum)
            }
            Self::Not(id) => (!ctx.has_buff(*id)).then_some(0),
            Self::Single(id) => ctx.stacks_of(*id),
        }
    }

    pub fn get_stacks_or_edit(&self, ctx: &RenderContext, state: &RenderState) -> Option<i32> {
        if state.edit {
            Some(1)
        } else {
            self.get_stacks(ctx)
        }
    }

    pub fn render_options(&mut self, ui: &Ui) {}
}
