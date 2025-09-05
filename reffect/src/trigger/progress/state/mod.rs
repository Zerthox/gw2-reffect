use super::ProgressActive;
use crate::{context::Context, render::enum_combo};
use const_default::ConstDefault;
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use std::fmt;

mod state_condition;
mod state_type;

pub use self::{state_condition::*, state_type::*};

#[derive(Debug, Default, ConstDefault, PartialEq, Clone, Serialize, Deserialize)]
pub struct TriggerState {
    /// State type to check.
    pub state_type: StateType,

    /// Condition for the state.
    pub condition: StateCondition,
}

impl TriggerState {
    pub fn is_active(&self, active: &ProgressActive, _ctx: &Context) -> bool {
        if let ProgressActive::Ability { info, .. } = active {
            let state_value = match self.state_type {
                StateType::Available => info.available,
                StateType::Pressed => info.pressed,
                StateType::Pending => info.pending,
            };
            match self.condition {
                StateCondition::True => state_value,
                StateCondition::False => !state_value,
            }
        } else {
            // For Buff and Fixed, states don't apply, so return true if condition is True
            // (meaning if no state check required)
            // matches!(self.condition, StateCondition::True)
            // well or just return false?
            false
        }
    }
}

impl fmt::Display for TriggerState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} is {:?}", self.state_type, self.condition)
    }
}

impl TriggerState {
    pub fn render_options(&mut self, ui: &Ui) -> bool {
        let mut changed = false;

        changed |= enum_combo(ui, "State", &mut self.state_type, ComboBoxFlags::empty()).is_some();

        changed |=
            enum_combo(ui, "Condition", &mut self.condition, ComboBoxFlags::empty()).is_some();

        changed
    }
}
