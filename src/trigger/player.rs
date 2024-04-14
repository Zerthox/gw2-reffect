use super::{CombatTrigger, Trigger};
use crate::{
    context::{Mount, Profession, RenderContext, Specialization},
    elements::HasOptions,
    util::enum_combo,
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
            && empty_or_contains(&self.profs, &ctx.player.prof)
            && empty_or_contains(&self.specs, &ctx.player.spec)
            && empty_or_contains(&self.mounts, &ctx.player.mount)
    }
}

impl HasOptions for PlayerTrigger {
    fn render_options(&mut self, ui: &Ui) {
        self.combat.render_options(ui);

        ui.spacing();
        ui.group(|| {
            for (i, prof) in self.profs.iter_mut().enumerate() {
                enum_combo(ui, format!("Prof {}", i + 1), prof, ComboBoxFlags::empty());
            }
            if ui.button("Add Prof") {
                self.profs.push(Profession::default());
            }
            ui.same_line();
            if ui.button("Remove Prof") {
                self.profs.pop();
            }
        });

        ui.spacing();
        ui.group(|| {
            for (i, spec) in self.specs.iter_mut().enumerate() {
                enum_combo(ui, format!("Spec {}", i + 1), spec, ComboBoxFlags::empty());
            }
            if ui.button("Add Spec") {
                self.specs.push(Specialization::default());
            }
            ui.same_line();
            if ui.button("Remove Spec") {
                self.specs.pop();
            }
        });

        ui.spacing();
        ui.group(|| {
            for (i, mount) in self.mounts.iter_mut().enumerate() {
                enum_combo(
                    ui,
                    format!("Mount {}", i + 1),
                    mount,
                    ComboBoxFlags::empty(),
                );
            }
            if ui.button("Add Mount") {
                self.mounts.push(Mount::default());
            }
            ui.same_line();
            if ui.button("Remove Mount") {
                self.mounts.pop();
            }
        });
    }
}

fn empty_or_contains<T>(slice: &[T], el: &T) -> bool
where
    T: PartialEq,
{
    slice.is_empty() || slice.contains(el)
}
