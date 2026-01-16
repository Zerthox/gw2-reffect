use super::TraitRequirement;
use crate::{
    action::Action,
    context::{Context, ProfSelection, Profession, Specialization, Update},
    render::{enum_combo_bitflags, helper, input_skill_id, input_trait_id},
    serde::bitflags,
    trigger::{MemoizedTrigger, TriggerMode},
};
use const_default::ConstDefault;
use enumflags2::BitFlags;
use nexus::imgui::{ComboBoxFlags, InputTextFlags, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct BuildTrigger {
    /// Build traits.
    pub traits: Vec<TraitRequirement>,

    /// Trigger logic mode for traits.
    pub trait_mode: TriggerMode,

    /// Build profession.
    #[serde(skip_serializing)]
    #[serde(with = "bitflags")]
    #[cfg_attr(feature = "schema", schemars(with = "bitflags::Schema<Profession>"))]
    profs: BitFlags<Profession>, // TODO: remove after grace period

    /// Build specialization.
    #[serde(with = "bitflags")]
    #[cfg_attr(
        feature = "schema",
        schemars(with = "bitflags::Schema<Specialization>")
    )]
    pub specs: BitFlags<Specialization>,

    /// Selected skills.
    pub skill_selections: Vec<u32>,

    /// Trigger logic mode for selected skills.
    pub skill_selections_mode: TriggerMode,

    /// Profession-specific selections.
    #[serde(with = "bitflags")]
    #[cfg_attr(feature = "schema", schemars(with = "bitflags::Schema<ProfSelection>"))]
    pub prof_selections: BitFlags<ProfSelection>,

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

    fn skill_selections_active(&self, ctx: &Context) -> bool {
        if let Ok(build) = ctx.player.build.as_ref() {
            self.skill_selections_mode
                .check_slice(&self.skill_selections, |id| {
                    build.skill_selections.contains(id)
                })
        } else {
            true
        }
    }

    fn prof_selections_active(&self, ctx: &Context) -> bool {
        let build = ctx.player.build.as_ref();
        TriggerMode::Any.check_flags_optional(
            self.prof_selections,
            build.map(|build| build.prof_selections).ok(),
        )
    }

    fn render_trait_options(&mut self, ui: &Ui) -> bool {
        let _id = ui.push_id("trait");
        let mut changed = false;

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

        changed
    }

    fn render_skill_options(&mut self, ui: &Ui) -> bool {
        let _id = ui.push_id("skill");
        let mut changed = false;

        changed |= self.skill_selections_mode.render_options(ui, "Skill Mode");

        let mut action = Action::new();
        for (i, id) in self.skill_selections.iter_mut().enumerate() {
            let _id = ui.push_id(i as i32);
            changed |= action.input_with_buttons(ui, i, || {
                input_skill_id(ui, "##id", id, InputTextFlags::empty())
            });
            ui.same_line();
            ui.text(format!("Skill Id {}", i + 1));

            if i == 0 {
                helper(ui, || {
                    ui.text("Skill selections in build (slot skills, skill overrides)");
                    ui.text("Can be found on the wiki, same as in GW2 API");
                    ui.text("Supports pasting chat links");
                });
            }
        }
        changed |= action.perform(&mut self.skill_selections);

        if ui.button("Add Skill") {
            self.skill_selections.push(0);
            changed = true;
        }

        changed
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

        changed |= self.render_trait_options(ui);

        changed |= self.render_skill_options(ui);

        changed |= enum_combo_bitflags(
            ui,
            "Prof Selections",
            &mut self.prof_selections,
            ComboBoxFlags::HEIGHT_LARGE,
        );

        if changed {
            // ensure fresh state after changed
            self.update(ctx);
        }

        changed
    }
}

impl MemoizedTrigger for BuildTrigger {
    fn memoized_state(&mut self) -> &mut bool {
        &mut self.active
    }

    fn needs_update(&self, ctx: &Context) -> bool {
        ctx.has_update(Update::Identity | Update::Traits)
    }

    fn resolve_active(&mut self, ctx: &Context) -> bool {
        self.specs_active(ctx)
            && self.traits_active(ctx)
            && self.skill_selections_active(ctx)
            && self.prof_selections_active(ctx)
    }
}

impl ConstDefault for BuildTrigger {
    const DEFAULT: Self = Self {
        traits: Vec::new(),
        trait_mode: TriggerMode::All,
        profs: BitFlags::EMPTY,
        specs: BitFlags::EMPTY,
        skill_selections: Vec::new(),
        skill_selections_mode: TriggerMode::All,
        prof_selections: BitFlags::EMPTY,
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
