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

    /// Relevant trigger, parent or own.
    trigger: Option<&'p ProgressTrigger>,

    /// Whether to force update children.
    force: bool,
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
            trigger: None,
            force: false,
        }
    }

    #[must_use]
    fn update_and_push(
        &self,
        filter: &mut FilterTrigger,
        trigger: &'p mut ProgressTrigger,
    ) -> Option<Self> {
        let Self {
            ctx,
            trigger: parent,
            force,
        } = *self;

        let force = force || filter.needs_update(ctx);
        filter.update_if_need(ctx);

        if !ctx.edit.is_editing() && !filter.allow_child_update() {
            return None;
        }

        if force {
            trigger.force_update(ctx, parent);
        } else {
            trigger.update_if_need(ctx, parent);
        }
        Some(Self {
            ctx: self.ctx,
            trigger: Some(trigger),
            force,
        })
    }

    fn active(&self) -> Option<&'p ProgressActive> {
        self.trigger?.active()
    }
}

impl VisitMut for Updater<'_, '_> {
    fn visit_pack(&mut self, pack: &mut Pack) {
        let Pack {
            common, elements, ..
        } = pack;
        if let Some(mut child) = self.update_and_push(&mut common.filter, &mut common.trigger) {
            child.visit_elements(elements);
        }
    }

    fn visit_element(&mut self, element: &mut Element) {
        let Element { common, kind } = element;
        if let Some(mut child) = self.update_and_push(&mut common.filter, &mut common.trigger) {
            child.visit_element_type(kind);
        }
    }

    fn visit_list_icon(&mut self, list_icon: &mut ListIcon) {
        if let Some(mut child) = self.update_and_push(&mut list_icon.filter, &mut list_icon.trigger)
        {
            child.visit_icon(&mut list_icon.icon);
        }
    }

    fn visit_icon(&mut self, icon: &mut Icon) {
        icon.props.update(self.ctx, self.active(), self.force);
    }

    fn visit_text(&mut self, text: &mut Text) {
        if self.force {
            text.reprocess_next_frame();
        }
        text.props.update(self.ctx, self.active(), self.force);
    }

    fn visit_bar(&mut self, bar: &mut Bar) {
        bar.props.update(self.ctx, self.active(), self.force);
    }
}
