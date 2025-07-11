mod condition;
mod filter;
mod map;
mod player;
mod progress;

pub use self::{condition::*, filter::*, map::*, player::*, progress::*};

use crate::{context::Context, elements::RenderCtx};
use enumflags2::{BitFlag, BitFlags};

// TODO: parametric return type?
pub trait Trigger {
    fn is_active_or_edit(&mut self, ctx: &RenderCtx) -> bool {
        ctx.is_edited() || (!ctx.edit.is_editing() && self.is_active(ctx))
    }

    fn is_active(&mut self, ctx: &Context) -> bool;
}

pub fn check_bitflags<T>(flags: BitFlags<T>, value: impl Into<BitFlags<T>>) -> bool
where
    T: Copy + BitFlag,
{
    flags.is_empty() || flags.intersects(value)
}

pub fn check_bitflags_optional<T>(flags: BitFlags<T>, value: Option<impl Into<BitFlags<T>>>) -> bool
where
    T: Copy + BitFlag,
{
    value
        .map(|value| check_bitflags(flags, value))
        .unwrap_or(true)
}
