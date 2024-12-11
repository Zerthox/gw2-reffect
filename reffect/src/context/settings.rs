use super::{Context, OWN_INTERVAL, PLAYER_INTERVAL};
use crate::settings::icon::IconSettings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextSettings {
    pub save_on_unload: bool,
    pub edit_during_combat: bool,
    pub edit_show_all: bool,
    pub font: Option<String>,

    #[serde(alias = "buffs_interval")]
    pub own_interval: u32,

    pub player_interval: u32,

    pub icon: IconSettings,
}

impl Default for ContextSettings {
    fn default() -> Self {
        Self {
            save_on_unload: true,
            edit_during_combat: false,
            edit_show_all: false,
            font: None,
            own_interval: OWN_INTERVAL,
            player_interval: PLAYER_INTERVAL,
            icon: IconSettings::default(),
        }
    }
}

impl Context {
    pub fn settings(&self) -> ContextSettings {
        ContextSettings {
            save_on_unload: self.save_on_unload,
            edit_during_combat: self.edit.during_combat,
            edit_show_all: self.edit.show_all,
            font: self.font.name().clone(),
            own_interval: self.own_interval.frequency,
            player_interval: self.player_interval.frequency,
            icon: self.icon_settings.clone(),
        }
    }

    pub fn load(&mut self, settings: ContextSettings) {
        let ContextSettings {
            save_on_unload: save_unload,
            edit_during_combat,
            edit_show_all,
            font,
            own_interval,
            player_interval,
            icon,
        } = settings;
        self.save_on_unload = save_unload;
        self.edit.during_combat = edit_during_combat;
        self.edit.show_all = edit_show_all;
        self.font.load(font);
        self.own_interval.frequency = own_interval;
        self.player_interval.frequency = player_interval;
        self.icon_settings = icon;
    }
}
