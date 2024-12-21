use crate::{
    action::Action,
    context::{Context, ContextUpdate},
    internal::Traits,
    render::RenderOptions,
    render_util::{helper, input_trait_id},
    trigger::memo::MemoizedTrigger,
};
use nexus::imgui::{InputTextFlags, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct TraitTrigger {
    pub traits: Vec<TraitRequirement>,

    #[serde(skip)]
    memo: Option<bool>,
}

impl MemoizedTrigger for TraitTrigger {
    fn memo(&mut self) -> &mut Option<bool> {
        &mut self.memo
    }

    fn needs_update(&self, ctx: &Context) -> bool {
        ctx.has_update(ContextUpdate::Player)
    }

    fn is_active_current(&mut self, ctx: &Context) -> bool {
        ctx.player
            .traits()
            .map(|traits| self.traits.iter().all(|req| req.is_met(traits)))
            .unwrap_or(false)
    }
}

impl RenderOptions for TraitTrigger {
    fn render_options(&mut self, ui: &Ui, _ctx: &Context) {
        let _id = ui.push_id("trait");
        let mut action = Action::new();
        for (i, req) in self.traits.iter_mut().enumerate() {
            let _id = ui.push_id(i as i32);
            action.input_with_buttons(ui, i, || {
                let [start, _] = ui.cursor_pos();
                let width = ui.calc_item_width();

                ui.checkbox("##present", &mut req.present);

                ui.same_line();
                let moved = ui.cursor_pos()[0] - start;
                ui.set_next_item_width(width - moved);
                input_trait_id(ui, "##id", &mut req.id, InputTextFlags::empty());
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
        action.perform(&mut self.traits);
        if ui.button("Add Trait Id") {
            self.traits.push(TraitRequirement::default());
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
