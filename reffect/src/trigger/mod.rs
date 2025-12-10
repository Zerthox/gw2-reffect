mod ability;
mod condition;
mod filter;
mod map;
mod mode;
mod player;
mod progress;

pub use self::{ability::*, condition::*, filter::*, map::*, mode::*, player::*, progress::*};

use crate::context::Context;

// TODO: parametric return type?
pub trait Trigger {
    fn is_active(&mut self, ctx: &Context) -> bool;
}
