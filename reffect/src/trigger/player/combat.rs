use super::Trigger;
use crate::context::Context;
use const_default::ConstDefault;
use nexus::imgui::{Selectable, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, ConstDefault, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
#[repr(transparent)]
pub struct CombatTrigger(Option<bool>);

impl CombatTrigger {
    pub const VALUES: &[Self] = &[Self(None), Self(Some(true)), Self(Some(false))];

    pub fn label(&self) -> &'static str {
        match self.0 {
            Some(true) => "In Combat",
            Some(false) => "Out of Combat",
            None => "Always",
        }
    }
}

impl Trigger for CombatTrigger {
    fn is_active(&mut self, ctx: &Context) -> bool {
        match self.0 {
            Some(combat) => combat == ctx.ui.combat,
            None => true,
        }
    }
}

impl CombatTrigger {
    pub fn render_options(&mut self, ui: &Ui) {
        if let Some(_token) = ui.begin_combo("Combat", self.label()) {
            for value in Self::VALUES.iter().copied() {
                let selected = value == *self;
                if Selectable::new(value.label()).selected(selected).build(ui) {
                    *self = value;
                }
                if selected {
                    ui.set_item_default_focus();
                }
            }
        }
    }
}
