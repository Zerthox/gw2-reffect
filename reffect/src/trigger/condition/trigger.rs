use super::ProgressActive;
use crate::{
    context::{Context, Updateable},
    enums::check_variant_array,
    render::{Validation, enum_combo},
    trigger::{
        AbilityInfoTrigger, MapTrigger, PlayerTrigger, ProgressSource, ProgressThreshold,
        ResourceInfoTrigger,
    },
};
use const_default::ConstDefault;
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use std::{fmt, mem};
use strum::{AsRefStr, EnumCount, EnumIter, IntoStaticStr, VariantArray};

// TODO: add target affinity (target triggers only)

/// Condition trigger.
#[derive(
    Debug, Clone, PartialEq, AsRefStr, IntoStaticStr, EnumIter, EnumCount, Serialize, Deserialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum ConditionTrigger {
    #[strum(serialize = "Trigger Threshold")]
    ProgressThreshold(ProgressThreshold),

    #[strum(serialize = "Resource Info")]
    ResourceInfo(ResourceInfoTrigger),

    #[serde(alias = "AbilityState")]
    #[strum(serialize = "Ability Info")]
    AbilityInfo(AbilityInfoTrigger),

    Player(PlayerTrigger),

    Map(MapTrigger),
}

impl VariantArray for ConditionTrigger {
    const VARIANTS: &'static [Self] = &[
        Self::ProgressThreshold(ProgressThreshold::DEFAULT),
        Self::ResourceInfo(ResourceInfoTrigger::DEFAULT),
        Self::AbilityInfo(AbilityInfoTrigger::DEFAULT),
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
    /// Checks whether the condition is active.
    pub fn is_active(&self, ctx: &Context, active: &ProgressActive) -> bool {
        match self {
            Self::ProgressThreshold(threshold) => threshold.is_met(active, ctx),
            Self::ResourceInfo(resource) => resource.is_active(active),
            Self::AbilityInfo(ability) => ability.is_active(active),
            Self::Player(player) => player.is_active(ctx),
            Self::Map(map) => map.is_active(),
        }
    }

    /// Checks whether the conditions are the same type.
    pub fn is_same_type(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }

    /// Validates the source for the condition.
    pub fn validate_source(&self, source: &ProgressSource) -> Validation<&'static str> {
        match self {
            Self::ResourceInfo(_) => ResourceInfoTrigger::validate(source),
            Self::AbilityInfo(_) => AbilityInfoTrigger::validate(source),
            Self::ProgressThreshold(_) | Self::Player(_) | Self::Map(_) => Validation::Ok,
        }
    }

    /// Renders condition trigger options.
    pub fn render_options(&mut self, ui: &Ui, ctx: &Context, source: &ProgressSource) {
        let valid = self.validate_source(source);
        valid.for_item(ui, || {
            enum_combo(ui, "Condition", self, ComboBoxFlags::empty())
        });

        match self {
            Self::ProgressThreshold(threshold) => {
                threshold.render_options(ui);
            }
            Self::ResourceInfo(resource) => {
                resource.render_options(ui);
            }
            Self::AbilityInfo(ability) => {
                ability.render_options(ui);
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
            Self::ProgressThreshold(_) | Self::AbilityInfo(_) | Self::ResourceInfo(_) => false,
            Self::Player(player) => player.needs_update(ctx),
            Self::Map(map) => map.needs_update(ctx),
        }
    }

    fn force_update(&mut self, ctx: &Context) {
        match self {
            Self::ProgressThreshold(_) | Self::AbilityInfo(_) | Self::ResourceInfo(_) => {}
            Self::Player(player) => player.force_update(ctx),
            Self::Map(map) => map.force_update(ctx),
        }
    }

    fn update_if_need(&mut self, ctx: &Context) {
        match self {
            Self::ProgressThreshold(_) | Self::AbilityInfo(_) | Self::ResourceInfo(_) => {}
            Self::Player(player) => player.update_if_need(ctx),
            Self::Map(map) => map.update_if_need(ctx),
        }
    }
}

impl fmt::Display for ConditionTrigger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ProgressThreshold(threshold) => threshold.fmt(f),
            Self::ResourceInfo(resource) => resource.fmt(f),
            Self::AbilityInfo(ability_state) => ability_state.fmt(f),
            Self::Player(_) => write!(f, "Player"),
            Self::Map(_) => write!(f, "Map"),
        }
    }
}
