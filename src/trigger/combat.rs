use super::Trigger;
use crate::{context::RenderContext, traits::RenderOptions};
use nexus::imgui::{Selectable, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
#[repr(transparent)]
pub struct CombatTrigger(Option<bool>);

impl CombatTrigger {
    fn label(tristate: Option<bool>) -> &'static str {
        match tristate {
            Some(true) => "In Combat",
            Some(false) => "Out of Combat",
            None => "Always",
        }
    }
}

impl Trigger for CombatTrigger {
    fn is_active(&self, ctx: &RenderContext) -> bool {
        self.0.map(|combat| combat == ctx.ui.combat).unwrap_or(true)
    }
}

impl RenderOptions for CombatTrigger {
    fn render_options(&mut self, ui: &Ui) {
        if let Some(_token) = ui.begin_combo("Combat", Self::label(self.0)) {
            const VALUES: &[Option<bool>] = &[None, Some(true), Some(false)];

            for value in VALUES.iter().copied() {
                let selected = value == self.0;
                if Selectable::new(Self::label(value))
                    .selected(selected)
                    .build(ui)
                {
                    self.0 = value;
                }
                if selected {
                    ui.set_item_default_focus();
                }
            }
        }
    }
}
