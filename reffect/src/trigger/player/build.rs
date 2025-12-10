use super::TraitRequirement;
use crate::{
    action::Action,
    context::{Context, Profession, Specialization, Update},
    render::{enum_combo_bitflags, helper, input_trait_id},
    serde::bitflags,
    trigger::{TriggerMode, Trigger},
};
use const_default::ConstDefault;
use enumflags2::BitFlags;
use nexus::imgui::{ComboBoxFlags, InputTextFlags, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct BuildTrigger {
    pub traits: Vec<TraitRequirement>,
    pub trait_mode: TriggerMode,

    #[serde(skip_serializing)]
    #[serde(with = "bitflags")]
    profs: BitFlags<Profession>, // TODO: remove after grace period

    #[serde(with = "bitflags")]
    pub specs: BitFlags<Specialization>,

    #[serde(skip)]
    active: bool,
}

impl BuildTrigger {
    pub fn load(&mut self) {
        // translate old profs to specs if specs empty
        // TODO: remove after grace period
        if self.specs.is_empty() {
            for prof in self.profs.iter() {
                self.specs.insert(prof.specializations());
            }
        }
    }

    pub fn needs_update(&self, ctx: &Context) -> bool {
        ctx.has_update(Update::Identity | Update::Traits)
    }

    pub fn update(&mut self, ctx: &Context) {
        self.active = self.specs_active(ctx) && self.traits_active(ctx);
    }

    fn specs_active(&self, ctx: &Context) -> bool {
        TriggerMode::Any.check_flags_optional(self.specs, ctx.player.spec.ok())
    }

    fn traits_active(&self, ctx: &Context) -> bool {
        if let Ok(build) = ctx.player.build.as_ref() {
            self.trait_mode
                .check_slice(&self.traits, |req| req.is_met(&build.traits))
        } else {
            true
        }
    }

    pub fn render_options(&mut self, ui: &Ui, ctx: &Context) -> bool {
        let _id = ui.push_id("build");
        let mut changed = false;

        changed |= enum_combo_bitflags(
            ui,
            "Specialization",
            &mut self.specs,
            ComboBoxFlags::HEIGHT_LARGE,
        );

        changed |= self.trait_mode.render_options(ui, "Trait Mode");

        let mut action = Action::new();
        for (i, req) in self.traits.iter_mut().enumerate() {
            let _id = ui.push_id(i as i32);
            action.input_with_buttons(ui, i, || {
                let [start, _] = ui.cursor_pos();
                let width = ui.calc_item_width();

                changed |= ui.checkbox("##present", &mut req.present);

                ui.same_line();
                let moved = ui.cursor_pos()[0] - start;
                ui.set_next_item_width(width - moved);
                changed |= input_trait_id(ui, "##id", &mut req.id, InputTextFlags::empty());
            });
            ui.same_line();
            ui.text(format!("Trait Id {}", i + 1));

            if i == 0 {
                helper(ui, || {
                    ui.text("Can be found on the wiki, same as in GW2 API");
                    ui.text("Supports pasting chat links");
                    ui.text("Checkbox controls present or missing");
                });
            }
        }
        changed |= action.perform(&mut self.traits);

        if ui.button("Add Trait") {
            self.traits.push(TraitRequirement::default());
            changed = true;
        }

        if changed {
            // ensure fresh state after changed
            self.update(ctx);
        }

        changed
    }
}

impl Trigger for BuildTrigger {
    fn is_active(&mut self, ctx: &Context) -> bool {
        if self.needs_update(ctx) {
            self.update(ctx);
        }
        self.active
    }
}

impl ConstDefault for BuildTrigger {
    const DEFAULT: Self = Self {
        traits: Vec::new(),
        trait_mode: TriggerMode::All,
        profs: BitFlags::EMPTY,
        specs: BitFlags::EMPTY,
        active: true,
    };
}

impl Default for BuildTrigger {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prof_transition() {
        let mut trigger = BuildTrigger {
            profs: Profession::Guardian | Profession::Necromancer,
            ..Default::default()
        };
        trigger.load();
        assert_eq!(
            trigger.specs,
            Specialization::Guardian
                | Specialization::Dragonhunter
                | Specialization::Firebrand
                | Specialization::Willbender
                | Specialization::Luminary
                | Specialization::Necromancer
                | Specialization::Reaper
                | Specialization::Scourge
                | Specialization::Harbinger
                | Specialization::Ritualist
        );

        let mut trigger = BuildTrigger {
            profs: Profession::Guardian | Profession::Necromancer,
            specs: Specialization::Dragonhunter | Specialization::Reaper,
            ..Default::default()
        };
        trigger.load();
        assert_eq!(
            trigger.specs,
            Specialization::Dragonhunter | Specialization::Reaper
        );
    }
}
