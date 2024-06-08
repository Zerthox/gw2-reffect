mod buff;
mod buff_id;
mod buff_threshold;
mod combat;
mod map;
mod memo;
mod meta;
mod player;

pub use self::{buff::*, buff_id::*, buff_threshold::*, combat::*, map::*, meta::*, player::*};

use crate::{context::Context, elements::RenderState};

// TODO: parametric return type?
pub trait Trigger {
    fn is_active_or_edit(&mut self, ctx: &Context, state: &RenderState) -> bool {
        state.is_edit(ctx) || self.is_active(ctx)
    }

    fn is_active(&mut self, ctx: &Context) -> bool;
}
