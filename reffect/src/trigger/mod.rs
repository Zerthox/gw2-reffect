mod ability;
mod condition;
mod filter;
mod map;
mod player;
mod progress;

pub use self::{ability::*, condition::*, filter::*, map::*, player::*, progress::*};

use crate::context::Context;
use enumflags2::{BitFlag, BitFlags};

// TODO: parametric return type?
pub trait Trigger {
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
