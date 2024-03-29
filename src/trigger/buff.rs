use super::Trigger;
use crate::context::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuffTrigger {
    Single(u32),
    Any(Vec<u32>),
    All(Vec<u32>),
}

impl BuffTrigger {
    pub fn get_stacks(&self, ctx: &Context) -> Option<i32> {
        if ctx.edit {
            Some(1)
        } else {
            match self {
                Self::Single(id) => ctx.stacks_of(*id),
                Self::Any(ids) => ids.iter().copied().find_map(|id| ctx.stacks_of(id)),
                Self::All(ids) => ids.iter().copied().find_map(|id| ctx.stacks_of(id)),
            }
        }
    }
}

impl Trigger for BuffTrigger {
    fn is_active(&self, ctx: &Context) -> bool {
        ctx.edit
            || match self {
                Self::Single(id) => ctx.has_buff(*id),
                Self::Any(ids) => ids.iter().copied().any(|buff| ctx.has_buff(buff)),
                Self::All(ids) => ids.iter().copied().all(|buff| ctx.has_buff(buff)),
            }
    }
}

impl Default for BuffTrigger {
    fn default() -> Self {
        Self::Single(0)
    }
}
