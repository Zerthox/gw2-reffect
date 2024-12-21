use super::icon::IconSettings;
use crate::render_util::{helper, LoadedFont};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GeneralSettings {
    pub save_on_unload: bool,
    pub use_game_icons: bool,
    pub font: LoadedFont,
    pub icon: IconSettings,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            save_on_unload: true,
            use_game_icons: false,
            font: LoadedFont::empty(),
            icon: IconSettings::default(),
        }
    }
}

impl GeneralSettings {
    pub fn render_options(&mut self, ui: &Ui) {
        let Self {
            save_on_unload,
            use_game_icons,
            font,
            icon: _,
        } = self;
        ui.checkbox("Save pack changes on addon unload", save_on_unload);
        ui.checkbox("Reuse game icons (expertimental)", use_game_icons);
        helper(ui, || {
            ui.text("Expertimental, may cause memory leaks and/or crashes")
        });
        font.render_select(ui, "Font");
    }
}
