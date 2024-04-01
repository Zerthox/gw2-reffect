mod buff;
mod map;
mod pack;
mod player;

pub use self::{buff::*, map::*, pack::*, player::*};

use crate::context::RenderContext;

pub trait Trigger {
    fn is_active(&self, ctx: &RenderContext) -> bool;
}
