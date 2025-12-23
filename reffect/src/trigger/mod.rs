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
    /// Returns the current trigger state.
    fn is_active(&mut self, ctx: &Context) -> bool;
}

pub trait MemoizedTrigger {
    /// Returns the memoized state.
    fn memoized_state(&mut self) -> &mut bool;

    /// Checks whether updates are needed.
    fn needs_update(&self, ctx: &Context) -> bool;

    /// Resolves the current trigger state.
    fn resolve_active(&mut self, ctx: &Context) -> bool;

    /// Updates the memoized trigger state.
    fn update(&mut self, ctx: &Context) {
        *self.memoized_state() = self.resolve_active(ctx);
    }
}

impl<T> Trigger for T
where
    T: MemoizedTrigger,
{
    fn is_active(&mut self, ctx: &Context) -> bool {
        if self.needs_update(ctx) {
            self.update(ctx);
        }
        *self.memoized_state()
    }
}
