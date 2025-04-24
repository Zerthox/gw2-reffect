use super::VisitMut;
use crate::{
    context::{Context, ContextUpdate},
    elements::Pack,
    trigger::FilterTrigger,
};

#[derive(Debug, Clone)]
pub struct FilterUpdater<'ctx> {
    ctx: &'ctx Context,
}

impl<'ctx> FilterUpdater<'ctx> {
    pub fn update(ctx: &'ctx Context, pack: &mut Pack) {
        // map change needs deep update
        if ctx.has_update(ContextUpdate::Map) {
            Self { ctx }.visit_pack(pack);
        }
    }
}

impl VisitMut for FilterUpdater<'_> {
    fn visit_filter_trigger(&mut self, filter: &mut FilterTrigger) {
        filter.update(self.ctx);
    }
}
