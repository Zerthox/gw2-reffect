mod buff;
mod map;
mod meta;
mod player;

pub use self::{buff::*, map::*, meta::*, player::*};

use crate::context::RenderContext;

pub trait Trigger {
    fn is_active(&self, ctx: &RenderContext) -> bool;
}
