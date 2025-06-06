use super::VisitMut;
use crate::{context::Context, elements::Pack, trigger::FilterTrigger};

#[derive(Debug, Clone)]
pub struct FilterUpdater<'ctx> {
    ctx: &'ctx Context,
}

impl<'ctx> FilterUpdater<'ctx> {
    pub fn update(ctx: &'ctx Context, packs: &mut [Pack]) {
        log::debug!("Updating filters for map id {}", ctx.map.id);
        Self { ctx }.visit_packs(packs);
    }
}

impl VisitMut for FilterUpdater<'_> {
    fn visit_filter_trigger(&mut self, filter: &mut FilterTrigger) {
        filter.update(self.ctx);
    }
}
