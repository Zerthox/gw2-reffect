use super::{ProgressActive, ProgressSource, TriggerMode};
use crate::{
    context::ResourceType,
    named::Named,
    render::{Validation, enum_combo_bitflags},
    serde::bitflags,
};
use const_default::ConstDefault;
use enumflags2::BitFlags;
use itertools::Itertools;
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Resource info trigger.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct ResourceInfoTrigger {
    /// Resource types.
    #[serde(with = "bitflags")]
    #[serde(alias = "states")]
    #[cfg_attr(feature = "schema", schemars(with = "bitflags::Schema<ResourceType>"))]
    pub types: BitFlags<ResourceType>,
}

impl ResourceInfoTrigger {
    pub fn is_active(&self, active: &ProgressActive) -> bool {
        active
            .resource_type()
            .is_some_and(|resource_type| TriggerMode::Any.check_flags(self.types, resource_type))
    }

    pub fn validate(source: &ProgressSource) -> Validation<&'static str> {
        match source {
            ProgressSource::Health { .. }
            | ProgressSource::HealthReduction
            | ProgressSource::Barrier { .. }
            | ProgressSource::Defiance { .. }
            | ProgressSource::Endurance
            | ProgressSource::PrimaryResource
            | ProgressSource::SecondaryResource
            | ProgressSource::ResourceRate => Validation::Ok,
            ProgressSource::Inherit => {
                Validation::Warn("Inherited trigger source must be resource-like")
            }
            ProgressSource::Always
            | ProgressSource::Buff { .. }
            | ProgressSource::Ability { .. }
            | ProgressSource::SkillbarSlot { .. } => {
                Validation::Error("Condition requires a resource-like trigger source")
            }
        }
    }

    pub fn render_options(&mut self, ui: &Ui) -> bool {
        let mut changed = false;

        changed |= enum_combo_bitflags(ui, "Resources", &mut self.types, ComboBoxFlags::empty());

        changed
    }
}

impl ConstDefault for ResourceInfoTrigger {
    const DEFAULT: Self = Self {
        types: BitFlags::EMPTY,
    };
}

impl Default for ResourceInfoTrigger {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl fmt::Display for ResourceInfoTrigger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let types = if !self.types.is_empty() {
            self.types.iter().map(|info| info.short_name()).join(",")
        } else {
            "...".into()
        };
        write!(f, "Resource is {types}")
    }
}
