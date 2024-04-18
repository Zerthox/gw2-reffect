mod buff;
mod combat;
mod map;
mod memo;
mod meta;
mod player;

pub use self::{buff::*, combat::*, map::*, meta::*, player::*};

use crate::{context::RenderContext, elements::RenderState};

pub trait Trigger {
    fn is_active_or_edit(&mut self, ctx: &RenderContext, state: &RenderState) -> bool {
        state.edit || self.is_active(ctx)
    }

    fn is_active(&mut self, ctx: &RenderContext) -> bool;
}
