use super::ProgressActive;
use crate::{
    context::AbilityState,
    named::Named,
    render::{enum_combo, enum_combo_bitflags, helper},
    serde::bitflags,
};
use const_default::ConstDefault;
use enumflags2::{BitFlags, make_bitflags};
use itertools::Itertools;
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use std::fmt;

mod value;

pub use self::value::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AbilityStateTrigger {
    #[serde(with = "bitflags")]
    pub states: BitFlags<AbilityState>,

    pub condition: Value,
}

impl AbilityStateTrigger {
    pub fn is_active(&self, active: &ProgressActive) -> bool {
        self.condition.check(active.extra_state(), self.states)
    }

    pub fn render_options(&mut self, ui: &Ui) -> bool {
        let mut changed = false;

        changed |= enum_combo_bitflags(ui, "State", &mut self.states, ComboBoxFlags::empty());
        helper(ui, || {
            ui.text("Auto Attack: ability is set to auto-attack");
            ui.text("Pressed: ability is pressed");
            ui.text("Pending: ability is casting or queued");
        });

        changed |= enum_combo(ui, "Value", &mut self.condition, ComboBoxFlags::empty()).is_some();

        changed
    }
}

impl ConstDefault for AbilityStateTrigger {
    const DEFAULT: Self = Self {
        states: make_bitflags!(AbilityState::Pending),
        condition: Value::Any,
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
        write!(f, "State is {} {states}", self.condition.as_ref())
    }
}
