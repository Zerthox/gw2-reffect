use super::VisitMut;
use crate::{
    context::{Context, Update},
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
    pub fn load(ctx: &'ctx mut Context, packs: &mut [Pack]) {
        ctx.force_update();
        Self::update(ctx, packs);
    }

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

        let child_updates = filter.update(ctx, force);
        if child_updates.allow || ctx.edit.is_editing() {
            let force = force || child_updates.force;
            trigger.update(ctx, parent, force);

            Some(Self {
                ctx: self.ctx,
                trigger: Some(trigger),
                force,
            })
        } else {
            None
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        context::{Gear, PlayerInfo, Weapon},
        trigger::{ChildUpdates, ProgressSource, ProgressTrigger},
    };

    fn create_ctx() -> Context {
        let mut ctx = Context::disabled();
        ctx.player = PlayerInfo::empty();
        ctx
    }

    fn create_filter() -> FilterTrigger {
        let mut filter = FilterTrigger::default();
        filter.player.gear.weapons = Weapon::AxeMain.into();
        filter
    }

    #[test]
    fn filter_update() {
        let mut ctx = create_ctx();
        let mut filter = create_filter();

        ctx.force_update();
        let child_updates = filter.update(&ctx, false);
        assert_eq!(
            child_updates,
            ChildUpdates {
                allow: false,
                force: false
            }
        );
        assert_eq!(filter.is_active(&ctx), false);

        ctx.player.gear = Ok(Gear {
            weapons: Weapon::AxeMain | Weapon::AxeOff,
            ..Gear::empty()
        });
        ctx.updates = Update::PlayerGear.into();

        let child_updates = filter.update(&ctx, false);
        assert_eq!(
            child_updates,
            ChildUpdates {
                allow: true,
                force: true
            }
        );
        assert_eq!(filter.is_active(&ctx), true);
    }

    #[test]
    fn push() {
        let mut ctx = create_ctx();
        ctx.player.gear = Ok(Gear {
            weapons: Weapon::AxeMain | Weapon::AxeOff,
            ..Gear::empty()
        });

        let mut filter = create_filter();

        let mut trigger = ProgressTrigger::with(ProgressSource::PrimaryResource);
        let trigger_ptr = &raw const trigger;

        ctx.force_update();
        let parent = Updater::root(&ctx)
            .update_and_push(&mut filter, &mut trigger)
            .expect("no parent visit");

        let active_trigger = parent.trigger.expect("no trigger");
        assert_eq!(&raw const *active_trigger, trigger_ptr);
        assert_eq!(parent.force, true);

        let mut trigger = ProgressTrigger::with(ProgressSource::Inherit);
        let trigger_ptr = &raw const trigger;

        let child = parent
            .update_and_push(&mut FilterTrigger::default(), &mut trigger)
            .expect("no child visit");

        let active_trigger = child.trigger.expect("no trigger");
        assert_eq!(&raw const *active_trigger, trigger_ptr);
        assert_eq!(child.force, true);
    }
}
