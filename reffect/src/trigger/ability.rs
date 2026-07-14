use super::ProgressActive;
use crate::{
    context::AbilityInfo,
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

/// Ability info trigger.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct AbilityInfoTrigger {
    /// Ability info.
    #[serde(with = "bitflags")]
    #[serde(alias = "states")]
    #[cfg_attr(feature = "schema", schemars(with = "bitflags::Schema<AbilityInfo>"))]
    pub infos: BitFlags<AbilityInfo>,

    /// Trigger logic mode.
    #[serde(alias = "condition")]
    pub mode: TriggerMode,
}

impl AbilityInfoTrigger {
    pub fn is_present(&self, active: &ProgressActive) -> bool {
        self.mode.check_flags(self.infos, active.ability_info())
    }

    pub fn render_options(&mut self, ui: &Ui) -> bool {
        let mut changed = false;

        changed |= enum_combo_bitflags(ui, "Info", &mut self.infos, ComboBoxFlags::empty());
        helper(ui, || {
            ui.text("Auto Attack: ability is set to auto-attack");
            ui.text("Pressed: ability is pressed");
            ui.text("Pending: ability is casting or queued");
        });

        changed |= self.mode.render_options(ui, "Mode");

        changed
    }
}

impl ConstDefault for AbilityInfoTrigger {
    const DEFAULT: Self = Self {
        infos: make_bitflags!(AbilityInfo::Pending),
        mode: TriggerMode::Any,
    };
}

impl Default for AbilityInfoTrigger {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl fmt::Display for AbilityInfoTrigger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let infos = if !self.infos.is_empty() {
            self.infos.iter().map(|info| info.short_name()).join(",")
        } else {
            "...".into()
        };
        write!(f, "Is {} {infos}", self.mode.as_ref())
    }
}
