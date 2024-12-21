use super::GeneralSettings;
use crate::context::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextSettings {
    #[serde(flatten)]
    pub general: GeneralSettings,

    pub edit_during_combat: bool,
    pub edit_show_all: bool,

    #[serde(alias = "combat_interval")]
    #[serde(alias = "own_interval")]
    #[serde(alias = "buffs_interval")]
    pub state_interval: u32,

    pub player_interval: u32,
}

impl Default for ContextSettings {
    fn default() -> Self {
        Self {
            general: GeneralSettings::default(),
            edit_during_combat: false,
            edit_show_all: false,
            state_interval: Context::DEFAULT_STATE_INTERVAL,
            player_interval: Context::DEFAULT_PLAYER_INTERVAL,
        }
    }
}

impl From<&Context> for ContextSettings {
    fn from(ctx: &Context) -> Self {
        Self {
            general: ctx.settings.clone(),
            edit_during_combat: ctx.edit.during_combat,
            edit_show_all: ctx.edit.show_all,
            state_interval: ctx.state_interval.frequency,
            player_interval: ctx.player_interval.frequency,
        }
    }
}

impl ContextSettings {
    pub fn apply(self, ctx: &mut Context) {
        let Self {
            general,
            edit_during_combat,
            edit_show_all,
            state_interval,
            player_interval,
        } = self;
        ctx.settings = general;
        ctx.edit.during_combat = edit_during_combat;
        ctx.edit.show_all = edit_show_all;
        ctx.state_interval.frequency = state_interval;
        ctx.player_interval.frequency = player_interval;
    }
}
