use super::Trigger;
use crate::context::RenderContext;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum BuffTrigger {
    #[default]
    Always,

    Single(u32),

    Not(Box<BuffTrigger>),

    Any(Vec<u32>),

    All(Vec<u32>),
}

impl BuffTrigger {
    pub fn get_stacks(&self, ctx: &RenderContext) -> Option<i32> {
        if ctx.edit {
            Some(1)
        } else {
            match self {
                Self::Always => Some(0),
                Self::Single(id) => ctx.stacks_of(*id),
                Self::Not(inner) => (!inner.is_active(ctx)).then_some(0),
                Self::Any(ids) => {
                    let sum = ctx.stacks_of_summed(ids);
                    (sum > 0).then_some(sum)
                }
                Self::All(ids) => ctx.has_buffs_all(ids).then(|| ctx.stacks_of_summed(ids)),
            }
        }
    }
}

impl Trigger for BuffTrigger {
    fn is_active(&self, ctx: &RenderContext) -> bool {
        ctx.edit
            || match self {
                Self::Always => true,
                Self::Single(id) => ctx.has_buff(*id),
                Self::Not(inner) => !inner.is_active(ctx),
                Self::Any(ids) => ctx.has_buffs_any(ids),
                Self::All(ids) => ctx.has_buffs_all(ids),
            }
    }
}
