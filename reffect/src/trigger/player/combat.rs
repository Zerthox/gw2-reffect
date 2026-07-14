use crate::context::Context;
use const_default::ConstDefault;
use nexus::imgui::{Selectable, Ui};
use serde::{Deserialize, Serialize};

/// Combat state trigger.
#[derive(Debug, Default, ConstDefault, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
#[repr(transparent)]
pub struct CombatTrigger(Option<bool>);

impl CombatTrigger {
    /// All possible values.
    pub const VALUES: &[Self] = &[Self(None), Self(Some(true)), Self(Some(false))];

    /// Checks whether the combat trigger is active.
    pub fn is_active(&self, ctx: &Context) -> bool {
        match self.0 {
            Some(combat) => combat == ctx.ui.combat,
            None => true,
        }
    }

    /// Renders combat trigger options.
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

    /// Returns the corresponding select item label.
    fn label(&self) -> &'static str {
        match self.0 {
            Some(true) => "In Combat",
            Some(false) => "Out of Combat",
            None => "Always",
        }
    }
}
