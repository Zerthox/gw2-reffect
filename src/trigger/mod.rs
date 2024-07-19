mod combat;
mod filter;
mod map;
mod memo;
mod player;
mod progress;

pub use self::{combat::*, filter::*, map::*, player::*, progress::*};

use crate::{context::Context, elements::RenderState};

// TODO: parametric return type?
pub trait Trigger {
    fn is_active_or_edit(&mut self, ctx: &Context, state: &RenderState) -> bool {
        state.is_edit(ctx) || (!ctx.edit.is_editing() && self.is_active(ctx))
    }

    fn is_active(&mut self, ctx: &Context) -> bool;
}
