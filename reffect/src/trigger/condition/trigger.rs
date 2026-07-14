use super::ProgressActive;
use crate::{
    context::{Context, Updateable},
    enums::check_variant_array,
    render::{Validation, enum_combo},
    trigger::{AbilityInfoTrigger, MapTrigger, PlayerTrigger, ProgressSource, ProgressThreshold},
};
use const_default::ConstDefault;
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use std::{fmt, mem};
use strum::{AsRefStr, EnumCount, EnumIter, IntoStaticStr, VariantArray};

// TODO: add target affinity (target triggers only)
// TODO: add defiance state (defiance triggers only)
#[derive(
    Debug, Clone, PartialEq, AsRefStr, IntoStaticStr, EnumIter, EnumCount, Serialize, Deserialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum ConditionTrigger {
    #[strum(serialize = "Trigger Threshold")]
    ProgressThreshold(ProgressThreshold),

    #[serde(alias = "AbilityInfo")]
    #[strum(serialize = "Ability State")]
    AbilityState(AbilityInfoTrigger),

    Player(PlayerTrigger),

    Map(MapTrigger),
}

impl VariantArray for ConditionTrigger {
    const VARIANTS: &'static [Self] = &[
        Self::ProgressThreshold(ProgressThreshold::DEFAULT),
        Self::AbilityState(AbilityInfoTrigger::DEFAULT),
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
    pub fn is_active(&self, ctx: &Context, active: &ProgressActive) -> bool {
        match self {
            Self::ProgressThreshold(threshold) => threshold.is_met(active, ctx),
            Self::AbilityState(ability_state) => ability_state.is_present(active),
            Self::Player(player) => player.is_active(ctx),
            Self::Map(map) => map.is_active(),
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
                | ProgressSource::HealthReduction
                | ProgressSource::Barrier { .. }
                | ProgressSource::Defiance { .. }
                | ProgressSource::Endurance
                | ProgressSource::PrimaryResource
                | ProgressSource::SecondaryResource
                | ProgressSource::ResourceRate => {
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

impl Updateable for ConditionTrigger {
    fn needs_update(&self, ctx: &Context) -> bool {
        match self {
            Self::ProgressThreshold(_) | Self::AbilityState(_) => false,
            Self::Player(player) => player.needs_update(ctx),
            Self::Map(map) => map.needs_update(ctx),
        }
    }

    fn force_update(&mut self, ctx: &Context) {
        match self {
            Self::ProgressThreshold(_) | Self::AbilityState(_) => {}
            Self::Player(player) => player.force_update(ctx),
            Self::Map(map) => map.force_update(ctx),
        }
    }

    fn update_if_need(&mut self, ctx: &Context) {
        match self {
            Self::ProgressThreshold(_) | Self::AbilityState(_) => {}
            Self::Player(player) => player.update_if_need(ctx),
            Self::Map(map) => map.update_if_need(ctx),
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
