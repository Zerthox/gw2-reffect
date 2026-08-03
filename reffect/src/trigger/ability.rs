use super::{ProgressActive, ProgressSource, TriggerMode};
use crate::{
    context::AbilityInfo,
    named::Named,
    render::{Validation, enum_combo_bitflags, helper},
    serde::bitflags,
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
    pub fn is_active(&self, active: &ProgressActive) -> bool {
        self.mode.check_flags(self.infos, active.ability_info())
    }

    pub fn validate(source: &ProgressSource) -> Validation<&'static str> {
        match source {
            ProgressSource::Ability { .. } | ProgressSource::SkillbarSlot { .. } => Validation::Ok,
            ProgressSource::Inherit => {
                Validation::Warn("Inherited trigger source must be ability-like")
            }
            ProgressSource::Always
            | ProgressSource::Buff { .. }
            | ProgressSource::Health { .. }
            | ProgressSource::HealthReduction
            | ProgressSource::Barrier { .. }
            | ProgressSource::Defiance { .. }
            | ProgressSource::Endurance
            | ProgressSource::PrimaryResource
            | ProgressSource::SecondaryResource
            | ProgressSource::ResourceRate => {
                Validation::Error("Condition requires an ability-like trigger source")
            }
        }
    }

    pub fn render_options(&mut self, ui: &Ui) -> bool {
        let mut changed = false;

        changed |= enum_combo_bitflags(ui, "Info", &mut self.infos, ComboBoxFlags::empty());
        helper(ui, || {
            ui.text("Auto Attack: ability is set to auto-attack");
            ui.text("Pending: ability is queued or casting");
            ui.text("Pressed: ability is pressed");
            ui.text("Active: ability is passively active");
            ui.text("No Resources: missing resources to activate");
            ui.text("No Range: out of ability range");
            ui.text("Ground Targeted: ability uses ground targeting");
            ui.text("Ignore Recharge: ability can be activated while recharging");
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
        write!(f, "Ability is {} {infos}", self.mode.as_ref())
    }
}
