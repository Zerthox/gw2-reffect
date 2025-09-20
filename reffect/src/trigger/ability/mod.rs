use super::ProgressActive;
use crate::render::{enum_combo, helper};
use const_default::ConstDefault;
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use std::fmt;

mod state;

pub use self::state::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AbilityStateTrigger {
    pub ability_state: AbilityState,
    pub condition: bool,
}

impl AbilityStateTrigger {
    pub fn is_active(&self, active: &ProgressActive) -> bool {
        let state_value = match self.ability_state {
            AbilityState::Pressed => active.is_ability_pressed(),
            AbilityState::Pending => active.is_ability_pending(),
        };
        state_value == self.condition
    }
}

impl ConstDefault for AbilityStateTrigger {
    const DEFAULT: Self = Self {
        ability_state: AbilityState::DEFAULT,
        condition: true,
    };
}

impl Default for AbilityStateTrigger {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl fmt::Display for AbilityStateTrigger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Ability {} {:?}",
            if self.condition { "is" } else { "is not" },
            self.ability_state
        )
    }
}

impl AbilityStateTrigger {
    pub fn render_options(&mut self, ui: &Ui) -> bool {
        let mut changed = false;

        changed |=
            enum_combo(ui, "State", &mut self.ability_state, ComboBoxFlags::empty()).is_some();
        helper(ui, || {
            ui.text("Must use `Ability Recharge` Trigger");
            ui.text("");
            ui.text("Pressed: this ability is currently pressed");
            ui.text("Pending: this ability is in a queued/pending state");
        });

        changed |= ui.checkbox("Active", &mut self.condition);
        helper(ui, || {
            ui.text("Check this if the condition should trigger when the state is active. Leave unchecked if it should trigger when the state is not active.");
        });

        changed
    }
}
