use super::{Context, BUFFS_INTERVAL, PLAYER_INTERVAL};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextSettings {
    pub buffs_interval: f64,
    pub player_interval: f64,
}

impl Default for ContextSettings {
    fn default() -> Self {
        Self {
            buffs_interval: BUFFS_INTERVAL,
            player_interval: PLAYER_INTERVAL,
        }
    }
}

impl Context {
    pub fn settings(&self) -> ContextSettings {
        ContextSettings {
            buffs_interval: self.buffs_update.frequency,
            player_interval: self.player_update.frequency,
        }
    }

    pub fn load(&mut self, settings: ContextSettings) {
        self.replace_buffs_interval(settings.buffs_interval);
        self.replace_player_intervals(settings.player_interval);
    }
}
