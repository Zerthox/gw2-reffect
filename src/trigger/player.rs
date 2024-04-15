use super::{CombatTrigger, Trigger};
use crate::{
    context::{Mount, Profession, RenderContext, Specialization},
    render_util::enum_combo_check,
    traits::{Leaf, RenderOptions},
};
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PlayerTrigger {
    pub combat: CombatTrigger,
    pub profs: Vec<Profession>,
    pub specs: Vec<Specialization>,
    pub mounts: Vec<Mount>, // TODO: mount example?
}

impl Trigger for PlayerTrigger {
    fn is_active(&self, ctx: &RenderContext) -> bool {
        self.combat.is_active(ctx)
            && empty_or_contains_optional(&self.profs, ctx.player.prof.as_ref().ok())
            && empty_or_contains_optional(&self.specs, ctx.player.spec.as_ref().ok())
            && empty_or_contains(&self.mounts, &ctx.player.mount)
    }
}

impl Leaf for PlayerTrigger {
    fn load(&mut self) {
        self.profs.sort();
        self.specs.sort();
        self.mounts.sort();
    }
}

impl RenderOptions for PlayerTrigger {
    fn render_options(&mut self, ui: &Ui) {
        self.combat.render_options(ui);

        enum_combo_check(
            ui,
            "Profession",
            &mut self.profs,
            ComboBoxFlags::HEIGHT_LARGE,
        );

        enum_combo_check(
            ui,
            "Specialization",
            &mut self.specs,
            ComboBoxFlags::HEIGHT_LARGE,
        );
        enum_combo_check(ui, "Mount", &mut self.mounts, ComboBoxFlags::HEIGHT_LARGE);
    }
}

fn empty_or_contains<T>(slice: &[T], el: &T) -> bool
where
    T: Ord,
{
    slice.is_empty() || slice.binary_search(el).is_ok()
}

fn empty_or_contains_optional<T>(slice: &[T], el: Option<&T>) -> bool
where
    T: Ord,
{
    el.map(|el| empty_or_contains(slice, el)).unwrap_or(true)
}
