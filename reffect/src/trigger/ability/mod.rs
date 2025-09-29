use super::ProgressActive;
use crate::render::{enum_combo_bitflags, helper};
use crate::serde::bitflags;
use const_default::ConstDefault;
use enumflags2::BitFlags;
use nexus::imgui::{ComboBoxFlags, Ui};
use reffect_core::named::Named;
use serde::{Deserialize, Serialize};
use std::fmt;

mod state;

pub use self::state::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbilityStateTrigger {
    #[serde(with = "bitflags")]
    pub ability_state: BitFlags<AbilityState>,

    pub condition: bool,
}

impl AbilityStateTrigger {
    pub fn is_active(&self, active: &ProgressActive) -> bool {
        let mut flags = BitFlags::empty();
        if active.is_ability_pressed() {
            flags |= AbilityState::Pressed;
        }
        if active.is_ability_pending() {
            flags |= AbilityState::Pending;
        }
        self.ability_state.intersects(flags) == self.condition
    }
}

impl ConstDefault for AbilityStateTrigger {
    const DEFAULT: Self = Self {
        ability_state: BitFlags::EMPTY,
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
        let states: Vec<&str> = self.ability_state.iter().map(|s| s.name()).collect();
        write!(
            f,
            "State {} {}",
            if self.condition { "=" } else { "!=" },
            if states.is_empty() {
                "None".to_string()
            } else {
                states.join(", ")
            }
        )
    }
}

impl AbilityStateTrigger {
    pub fn render_options(&mut self, ui: &Ui) -> bool {
        let mut changed = false;

        changed |=
            enum_combo_bitflags(ui, "State", &mut self.ability_state, ComboBoxFlags::empty());

        helper(ui, || {
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
