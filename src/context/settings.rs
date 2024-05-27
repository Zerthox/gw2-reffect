use super::{Context, BUFFS_INTERVAL, PLAYER_INTERVAL};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextSettings {
    pub edit_during_combat: bool,
    pub buffs_interval: f64,
    pub player_interval: f64,
}

impl Default for ContextSettings {
    fn default() -> Self {
        Self {
            edit_during_combat: false,
            buffs_interval: BUFFS_INTERVAL,
            player_interval: PLAYER_INTERVAL,
        }
    }
}

impl Context {
    pub fn settings(&self) -> ContextSettings {
        ContextSettings {
            edit_during_combat: self.edit.during_combat,
            buffs_interval: self.buffs_interval.frequency,
            player_interval: self.player_interval.frequency,
        }
    }

    pub fn load(&mut self, settings: ContextSettings) {
        self.edit.during_combat = settings.edit_during_combat;
        self.replace_buffs_interval(settings.buffs_interval);
        self.replace_player_intervals(settings.player_interval);
    }
}
