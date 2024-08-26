use super::Trigger;
use crate::context::Context;

pub trait MemoizedTrigger {
    fn memo(&mut self) -> &mut Option<bool>;

    fn needs_update(&self, ctx: &Context) -> bool;

    fn is_active_current(&mut self, ctx: &Context) -> bool;

    fn update(&mut self, ctx: &Context) -> bool {
        let active = self.is_active_current(ctx);
        *self.memo().insert(active)
    }

    fn get(&mut self) -> Option<bool> {
        *self.memo()
    }

    fn is_active_memoized(&mut self, ctx: &Context) -> bool {
        if let Some(active) = self.get() {
            if self.needs_update(ctx) {
                self.update(ctx)
            } else {
                active
            }
        } else {
            self.update(ctx)
        }
    }

    fn clear(&mut self) {
        *self.memo() = None;
    }
}

impl<T> Trigger for T
where
    T: MemoizedTrigger,
{
    fn is_active(&mut self, ctx: &Context) -> bool {
        self.is_active_memoized(ctx)
    }
}
