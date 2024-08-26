use super::ProgressActive;
use crate::{
    context::{Context, EditState},
    render_util::{enum_combo, impl_static_variants},
    traits::RenderOptions,
    trigger::{MapTrigger, PlayerTrigger, ProgressThreshold, Trigger},
};
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{AsRefStr, EnumIter, IntoStaticStr};

#[derive(Debug, Clone, AsRefStr, IntoStaticStr, EnumIter, Serialize, Deserialize)]
pub enum ConditionTrigger {
    #[strum(serialize = "Progress Threshold")]
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
    fn render_options(&mut self, ui: &Ui, state: &mut EditState) {
        enum_combo(ui, "Condition", self, ComboBoxFlags::empty());

        match self {
            Self::ProgressThreshold(threshold) => threshold.render_options(ui, state),
            Self::Player(player) => player.render_options(ui, state),
            Self::Map(map) => {
                map.render_options(ui, state);
            }
        }
    }
}
