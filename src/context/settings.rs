use super::{Context, OWN_INTERVAL, PLAYER_INTERVAL};
use crate::render_util::Font;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextSettings {
    pub edit_during_combat: bool,
    pub edit_show_all: bool,
    pub font: Option<String>,

    #[serde(alias = "buffs_interval")]
    pub own_interval: u32,

    pub player_interval: u32,
}

impl Default for ContextSettings {
    fn default() -> Self {
        Self {
            edit_during_combat: false,
            edit_show_all: false,
            font: None,
            own_interval: OWN_INTERVAL,
            player_interval: PLAYER_INTERVAL,
        }
    }
}

impl Context {
    pub fn settings(&self) -> ContextSettings {
        ContextSettings {
            edit_during_combat: self.edit.during_combat,
            edit_show_all: self.edit.show_all,
            font: self.font.map(|font| font.name_owned()),
            own_interval: self.own_interval.frequency,
            player_interval: self.player_interval.frequency,
        }
    }

    pub fn load(&mut self, settings: ContextSettings) {
        let ContextSettings {
            edit_during_combat,
            edit_show_all,
            font,
            own_interval,
            player_interval,
        } = settings;
        self.edit.during_combat = edit_during_combat;
        self.edit.show_all = edit_show_all;
        self.font = font.and_then(Font::from_name_or_warn);
        self.own_interval.frequency = own_interval;
        self.player_interval.frequency = player_interval;
    }
}
