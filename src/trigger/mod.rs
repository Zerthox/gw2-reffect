mod combat;
mod map;
mod memo;
mod meta;
mod player;
mod progress;

pub use self::{combat::*, map::*, meta::*, player::*, progress::*};

use crate::{context::Context, elements::RenderState};

// TODO: parametric return type?
pub trait Trigger {
    fn is_active_or_edit(&mut self, ctx: &Context, state: &RenderState) -> bool {
        state.is_edit(ctx) || (!ctx.edit.is_editing() && self.is_active(ctx))
    }

    fn is_active(&mut self, ctx: &Context) -> bool;
}
