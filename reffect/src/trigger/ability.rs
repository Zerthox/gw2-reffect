use super::ProgressActive;
use crate::{
    context::AbilityState,
    named::Named,
    render::{enum_combo_bitflags, helper},
    serde::bitflags,
    trigger::TriggerMode,
};
use const_default::ConstDefault;
use enumflags2::{BitFlags, make_bitflags};
use itertools::Itertools;
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct AbilityStateTrigger {
    /// Ability states.
    #[serde(with = "bitflags")]
    #[cfg_attr(feature = "schema", schemars(with = "bitflags::Schema<AbilityState>"))]
    pub states: BitFlags<AbilityState>,

    /// Trigger logic mode.
    #[serde(alias = "condition")]
    pub mode: TriggerMode,
}

impl AbilityStateTrigger {
    pub fn is_active(&self, active: &ProgressActive) -> bool {
        self.mode.check_flags(self.states, active.ability_state())
    }

    pub fn render_options(&mut self, ui: &Ui) -> bool {
        let mut changed = false;

        changed |= enum_combo_bitflags(ui, "State", &mut self.states, ComboBoxFlags::empty());
        helper(ui, || {
            ui.text("Auto Attack: ability is set to auto-attack");
            ui.text("Pressed: ability is pressed");
            ui.text("Pending: ability is casting or queued");
        });

        changed |= self.mode.render_options(ui, "Mode");

        changed
    }
}

impl ConstDefault for AbilityStateTrigger {
    const DEFAULT: Self = Self {
        states: make_bitflags!(AbilityState::Pending),
        mode: TriggerMode::Any,
    };
}

impl Default for AbilityStateTrigger {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl fmt::Display for AbilityStateTrigger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let states = if !self.states.is_empty() {
            self.states.iter().map(|state| state.short_name()).join(",")
        } else {
            "...".into()
        };
        write!(f, "State is {} {states}", self.mode.as_ref())
    }
}
