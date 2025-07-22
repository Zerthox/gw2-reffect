use super::ProgressActive;
use crate::{
    context::Context,
    enums::check_variant_array,
    render::enum_combo,
    trigger::{MapTrigger, PlayerTrigger, ProgressThreshold, Trigger},
};
use const_default::ConstDefault;
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use std::{fmt, mem};
use strum::{AsRefStr, EnumCount, EnumIter, IntoStaticStr, VariantArray};

#[derive(
    Debug, Clone, PartialEq, AsRefStr, IntoStaticStr, EnumIter, EnumCount, Serialize, Deserialize,
)]
pub enum ConditionTrigger {
    #[strum(serialize = "Trigger Threshold")]
    ProgressThreshold(ProgressThreshold),

    Player(PlayerTrigger),

    Map(MapTrigger),
}

impl VariantArray for ConditionTrigger {
    const VARIANTS: &'static [Self] = &[
        Self::ProgressThreshold(ProgressThreshold::DEFAULT),
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
            Self::Player(player) => player.is_active(ctx),
            Self::Map(map) => map.is_active(ctx),
        }
    }

    pub fn is_same_type(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}

impl fmt::Display for ConditionTrigger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ProgressThreshold(threshold) => threshold.fmt(f),
            Self::Player(_) => write!(f, "Player"),
            Self::Map(_) => write!(f, "Map"),
        }
    }
}

impl ConditionTrigger {
    pub fn render_options(&mut self, ui: &Ui, ctx: &Context) {
        enum_combo(ui, "Condition", self, ComboBoxFlags::empty());

        match self {
            Self::ProgressThreshold(threshold) => {
                threshold.render_options(ui);
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
