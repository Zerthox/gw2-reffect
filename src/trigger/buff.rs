use super::Trigger;
use crate::context::RenderContext;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum BuffTrigger {
    #[default]
    Always,

    Not(Box<BuffTrigger>),

    Any(Vec<BuffTrigger>),

    All(Vec<BuffTrigger>),

    #[serde(untagged)]
    Single(u32),
}

impl BuffTrigger {
    pub fn get_stacks(&self, ctx: &RenderContext) -> Option<i32> {
        if ctx.edit {
            Some(1)
        } else {
            match self {
                Self::Always => Some(0),
                Self::Not(inner) => (!inner.is_active(ctx)).then_some(0),
                Self::Any(inner) => {
                    let mut iter = inner.iter().filter_map(|entry| entry.get_stacks(ctx));
                    iter.next().map(|first| first + iter.sum::<i32>())
                }
                Self::All(inner) => {
                    let mut sum = 0;
                    for entry in inner {
                        if let Some(stacks) = entry.get_stacks(ctx) {
                            sum += stacks;
                        } else {
                            return None;
                        }
                    }
                    Some(sum)
                }
                Self::Single(id) => ctx.stacks_of(*id),
            }
        }
    }
}

impl Trigger for BuffTrigger {
    fn is_active(&self, ctx: &RenderContext) -> bool {
        ctx.edit
            || match self {
                Self::Always => true,
                Self::Not(inner) => !inner.is_active(ctx),
                Self::Any(inner) => inner.iter().any(|entry| entry.is_active(ctx)),
                Self::All(inner) => inner.iter().all(|entry| entry.is_active(ctx)),
                Self::Single(id) => ctx.has_buff(*id),
            }
    }
}
