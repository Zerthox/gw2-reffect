use super::VisitMut;
use crate::{
    context::{Context, Update, Updateable},
    elements::{Bar, Common, Element, Icon, Pack, Text, list::ListIcon},
    profiling::measure,
    trigger::{FilterTrigger, ProgressActive, ProgressTrigger},
};

#[derive(Debug, Clone)]
pub struct Updater<'ctx, 'p> {
    ctx: &'ctx Context,
    current: Option<&'p ProgressActive>,
    parent: Option<&'p ProgressActive>,
}

impl<'ctx, 'p> Updater<'ctx, 'p> {
    pub fn update(ctx: &'ctx Context, packs: &mut [Pack]) {
        if ctx.has_update(Update::Map) {
            log::debug!("Map changed to {}", ctx.map.id);
        }
        measure(
            || Self::root(ctx).visit_packs(packs),
            |elapsed| {
                if ctx.has_any_update() {
                    log::trace!("Update took {elapsed:?} for {}", ctx.updates);
                }
            },
        );
    }

    fn root(ctx: &'ctx Context) -> Self {
        Self {
            ctx,
            current: None,
            parent: None,
        }
    }

    fn next(&self, current: &'p ProgressTrigger) -> Self {
        Self {
            ctx: self.ctx,
            parent: self.current,
            current: current.active(),
        }
    }

    fn visit_filter_and_progress(
        &self,
        filter: &mut FilterTrigger,
        progress: &mut ProgressTrigger,
    ) {
        let Self { ctx, parent, .. } = *self;

        let force = filter.needs_update(ctx);
        filter.update_if_need(ctx);

        if force {
            progress.force_update(ctx, parent);
        } else if filter.can_update_progress() {
            progress.update_if_need(ctx, parent);
        }
    }
}

impl VisitMut for Updater<'_, '_> {
    fn visit_pack(&mut self, pack: &mut Pack) {
        self.visit_common(&mut pack.common);
        self.next(&pack.common.trigger)
            .visit_elements(&mut pack.elements);
    }

    fn visit_element(&mut self, element: &mut Element) {
        self.visit_common(&mut element.common);
        self.next(&element.common.trigger)
            .visit_element_type(&mut element.kind);
    }

    fn visit_common(&mut self, common: &mut Common) {
        self.visit_filter_and_progress(&mut common.filter, &mut common.trigger);
    }

    fn visit_list_icon(&mut self, list_icon: &mut ListIcon) {
        self.visit_icon(&mut list_icon.icon);
        self.visit_filter_and_progress(&mut list_icon.filter, &mut list_icon.trigger);
    }

    fn visit_icon(&mut self, icon: &mut Icon) {
        icon.props.update(self.ctx, self.current);
    }

    fn visit_text(&mut self, text: &mut Text) {
        text.props.update(self.ctx, self.current);
    }

    fn visit_bar(&mut self, bar: &mut Bar) {
        bar.props.update(self.ctx, self.current);
    }
}
