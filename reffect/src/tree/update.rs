use super::VisitMut;
use crate::{
    context::{Context, Update, Updateable},
    elements::{Bar, Element, Icon, Pack, Text, list::ListIcon},
    profiling::measure,
    trigger::{FilterTrigger, ProgressActive, ProgressTrigger},
};

#[derive(Debug, Clone)]
pub struct Updater<'ctx, 'p> {
    /// Current context.
    ctx: &'ctx Context,

    /// Relevant progress, parent or own.
    active: Option<&'p ProgressActive>,
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
        Self { ctx, active: None }
    }

    #[must_use]
    fn update_and_push(
        &self,
        filter: &mut FilterTrigger,
        progress: &'p mut ProgressTrigger,
    ) -> Self {
        let Self {
            ctx,
            active: parent,
        } = *self;

        let force = filter.needs_update(ctx);
        filter.update_if_need(ctx);

        if force {
            progress.force_update(ctx, parent);
        } else if filter.can_update_progress() {
            progress.update_if_need(ctx, parent);
        }

        Self {
            ctx: self.ctx,
            active: progress.active(),
        }
    }
}

impl VisitMut for Updater<'_, '_> {
    fn visit_pack(&mut self, pack: &mut Pack) {
        let Pack {
            common, elements, ..
        } = pack;
        self.update_and_push(&mut common.filter, &mut common.trigger)
            .visit_elements(elements);
    }

    fn visit_element(&mut self, element: &mut Element) {
        let Element { common, kind } = element;
        self.update_and_push(&mut common.filter, &mut common.trigger)
            .visit_element_type(kind);
    }

    fn visit_list_icon(&mut self, list_icon: &mut ListIcon) {
        self.update_and_push(&mut list_icon.filter, &mut list_icon.trigger)
            .visit_icon(&mut list_icon.icon);
    }

    fn visit_icon(&mut self, icon: &mut Icon) {
        icon.props.update(self.ctx, self.active);
    }

    fn visit_text(&mut self, text: &mut Text) {
        text.props.update(self.ctx, self.active);
    }

    fn visit_bar(&mut self, bar: &mut Bar) {
        bar.props.update(self.ctx, self.active);
    }
}
