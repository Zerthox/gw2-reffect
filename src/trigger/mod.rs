mod condition;
mod filter;
mod map;
mod memo;
mod player;
mod progress;

pub use self::{condition::*, filter::*, map::*, player::*, progress::*};

use crate::{context::Context, elements::RenderState};
use enumflags2::{BitFlag, BitFlags};

// TODO: parametric return type?
pub trait Trigger {
    fn is_active_or_edit(&mut self, ctx: &Context, state: &RenderState) -> bool {
        state.is_edit(ctx) || (!ctx.edit.is_editing() && self.is_active(ctx))
    }

    fn is_active(&mut self, ctx: &Context) -> bool;
}

pub fn check_bitflags<T>(flags: BitFlags<T>, value: T) -> bool
where
    T: Copy + BitFlag,
{
    flags.is_empty() || flags.contains(value)
}

pub fn check_bitflags_optional<T>(flags: BitFlags<T>, value: Option<T>) -> bool
where
    T: Copy + BitFlag,
{
    value
        .map(|value| check_bitflags(flags, value))
        .unwrap_or(true)
}
