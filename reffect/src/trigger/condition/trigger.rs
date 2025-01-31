use super::ProgressActive;
use crate::{
    context::Context,
    render::{enum_combo, impl_static_variants, RenderOptions},
    trigger::{MapTrigger, PlayerTrigger, ProgressThreshold, Trigger},
};
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use std::{fmt, mem};
use strum::{AsRefStr, EnumIter, IntoStaticStr};

#[derive(Debug, Clone, PartialEq, AsRefStr, IntoStaticStr, EnumIter, Serialize, Deserialize)]
pub enum ConditionTrigger {
    #[strum(serialize = "Trigger Threshold")]
    ProgressThreshold(ProgressThreshold),

    Player(PlayerTrigger),

    Map(MapTrigger),
}

impl_static_variants!(ConditionTrigger);

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

impl Default for ConditionTrigger {
    fn default() -> Self {
        Self::ProgressThreshold(ProgressThreshold::default())
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

impl RenderOptions for ConditionTrigger {
    fn render_options(&mut self, ui: &Ui, ctx: &Context) {
        enum_combo(ui, "Condition", self, ComboBoxFlags::empty());

        match self {
            Self::ProgressThreshold(threshold) => threshold.render_options(ui, ctx),
            Self::Player(player) => player.render_options(ui, ctx),
            Self::Map(map) => {
                map.render_options(ui, ctx);
            }
        }
    }
}
