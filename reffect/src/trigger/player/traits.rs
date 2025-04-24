use crate::{
    action::Action,
    context::{Context, ContextUpdate},
    internal::Traits,
    render::{helper, input_trait_id, RenderOptions},
    trigger::Trigger,
};
use nexus::imgui::{InputTextFlags, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TraitTrigger {
    pub traits: Vec<TraitRequirement>,

    #[serde(skip)]
    active: bool,
}

impl TraitTrigger {
    pub fn update(&mut self, ctx: &Context) {
        self.active = self.resolve_active(ctx);
    }

    fn resolve_active(&self, ctx: &Context) -> bool {
        ctx.player
            .traits()
            .map(|traits| self.traits.iter().all(|req| req.is_met(traits)))
            .unwrap_or(false)
    }
}

impl Trigger for TraitTrigger {
    fn is_active(&mut self, ctx: &Context) -> bool {
        if ctx.has_update(ContextUpdate::Player) {
            self.update(ctx);
        }
        self.active
    }
}

impl RenderOptions<bool> for TraitTrigger {
    fn render_options(&mut self, ui: &Ui, ctx: &Context) -> bool {
        let _id = ui.push_id("trait");
        let mut changed = false;

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

        if ui.button("Add Trait Id") {
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

impl Default for TraitTrigger {
    fn default() -> Self {
        Self {
            traits: Vec::new(),
            active: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TraitRequirement {
    pub id: u32,
    pub present: bool,
}

impl TraitRequirement {
    pub fn is_met(&self, traits: &Traits) -> bool {
        let contains = traits.contains(&self.id);
        match self.present {
            true => contains,
            false => !contains,
        }
    }
}

impl Default for TraitRequirement {
    fn default() -> Self {
        Self {
            id: 0,
            present: true,
        }
    }
}
