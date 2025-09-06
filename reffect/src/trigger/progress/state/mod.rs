use super::ProgressActive;
use crate::{context::Context, render::enum_combo};
use const_default::ConstDefault;
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use std::fmt;

mod state_type;

pub use self::state_type::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TriggerState {
    pub state_type: StateType,
    pub condition: bool,
}

impl TriggerState {
    pub fn is_active(&self, active: &ProgressActive, _ctx: &Context) -> bool {
        let (available, pressed, pending) = active.state_info();
        let state_value = match self.state_type {
            StateType::Available => available,
            StateType::Pressed => pressed,
            StateType::Pending => pending,
        };
        state_value == self.condition
    }
}

impl ConstDefault for TriggerState {
    const DEFAULT: Self = Self {
        state_type: StateType::DEFAULT,
        condition: true,
    };
}

impl Default for TriggerState {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl fmt::Display for TriggerState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:?} is {}",
            self.state_type,
            if self.condition { "True" } else { "False" }
        )
    }
}

impl TriggerState {
    pub fn render_options(&mut self, ui: &Ui) -> bool {
        let mut changed = false;

        changed |= enum_combo(ui, "State", &mut self.state_type, ComboBoxFlags::empty()).is_some();
        changed |= ui.checkbox("Required", &mut self.condition);

        changed
    }
}
