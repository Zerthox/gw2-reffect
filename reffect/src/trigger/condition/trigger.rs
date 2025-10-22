use super::ProgressActive;
use crate::{
    context::Context,
    render::{Validation, enum_combo},
    trigger::{
        AbilityStateTrigger, MapTrigger, PlayerTrigger, ProgressSource, ProgressThreshold, Trigger,
    },
};
use const_default::ConstDefault;
use nexus::imgui::{ComboBoxFlags, Ui};
use reffect_core::enums::check_variant_array;
use serde::{Deserialize, Serialize};
use std::{fmt, mem};
use strum::{AsRefStr, EnumCount, EnumIter, IntoStaticStr, VariantArray};

#[derive(
    Debug, Clone, PartialEq, AsRefStr, IntoStaticStr, EnumIter, EnumCount, Serialize, Deserialize,
)]
pub enum ConditionTrigger {
    #[strum(serialize = "Trigger Threshold")]
    ProgressThreshold(ProgressThreshold),

    #[serde(alias = "AbilityInfo")]
    #[strum(serialize = "Ability State")]
    AbilityState(AbilityStateTrigger),

    Player(PlayerTrigger),

    Map(MapTrigger),
}

impl VariantArray for ConditionTrigger {
    const VARIANTS: &'static [Self] = &[
        Self::ProgressThreshold(ProgressThreshold::DEFAULT),
        Self::AbilityState(AbilityStateTrigger::DEFAULT),
        Self::Player(PlayerTrigger::DEFAULT),
        Self::Map(MapTrigger::DEFAULT),
    ];
}

const _: () = check_variant_array::<ConditionTrigger>();

impl ConstDefault for ConditionTrigger {
    const DEFAULT: Self = Self::ProgressThreshold(ProgressThreshold::DEFAULT);
}

impl Default for ConditionTrigger {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl ConditionTrigger {
    pub fn is_active(&mut self, ctx: &Context, active: &ProgressActive) -> bool {
        match self {
            Self::ProgressThreshold(threshold) => threshold.is_met(active, ctx),
            Self::AbilityState(ability_state) => ability_state.is_active(active),
            Self::Player(player) => player.is_active(ctx),
            Self::Map(map) => map.is_active(ctx),
        }
    }

    pub fn is_same_type(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }

    pub fn validate_source(&self, source: &ProgressSource) -> Validation<&'static str> {
        match self {
            Self::AbilityState(_) => match source {
                ProgressSource::Ability { .. } | ProgressSource::SkillbarSlot { .. } => {
                    Validation::Ok
                }
                ProgressSource::Inherit => {
                    Validation::Warn("Inherited trigger source must be ability-like")
                }
                ProgressSource::Always
                | ProgressSource::Buff { .. }
                | ProgressSource::Health { .. }
                | ProgressSource::Barrier { .. }
                | ProgressSource::Defiance { .. }
                | ProgressSource::Endurance
                | ProgressSource::PrimaryResource
                | ProgressSource::SecondaryResource => {
                    Validation::Error("Condition requires an ability-like trigger source")
                }
            },
            Self::ProgressThreshold(_) | Self::Player(_) | Self::Map(_) => Validation::Ok,
        }
    }

    pub fn render_options(&mut self, ui: &Ui, ctx: &Context, source: &ProgressSource) {
        let valid = self.validate_source(source);
        valid.for_item(ui, || {
            enum_combo(ui, "Condition", self, ComboBoxFlags::empty())
        });

        match self {
            Self::ProgressThreshold(threshold) => {
                threshold.render_options(ui);
            }
            Self::AbilityState(ability_state) => {
                ability_state.render_options(ui);
            }
            Self::Player(player) => {
                player.render_options(ui, ctx);
            }
            Self::Map(map) => {
                map.render_options(ui, ctx);
            }
        }
    }
}

impl fmt::Display for ConditionTrigger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ProgressThreshold(threshold) => threshold.fmt(f),
            Self::AbilityState(ability_state) => ability_state.fmt(f),
            Self::Player(_) => write!(f, "Player"),
            Self::Map(_) => write!(f, "Map"),
        }
    }
}
