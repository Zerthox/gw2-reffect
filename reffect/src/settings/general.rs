use super::icon::IconSettings;
use crate::render_util::{helper, input_seconds, LoadedFont};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GeneralSettings {
    pub save_on_unload: bool,
    pub use_game_icons: bool,
    pub format: FormatSettings,
    pub font: LoadedFont,
    pub icon: IconSettings,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            save_on_unload: true,
            use_game_icons: false,
            format: FormatSettings::default(),
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
            format,
            font,
            icon: _,
        } = self;
        ui.checkbox("Save pack changes on addon unload", save_on_unload);

        ui.checkbox("Reuse game icons (expertimental)", use_game_icons);
        helper(ui, || {
            ui.text("Expertimental, may cause memory leaks and/or crashes")
        });

        format.render_options(ui);

        font.render_select(ui, "Font");
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FormatSettings {
    pub minutes_threshold: u32,
    pub millis_threshold: u32,
}

impl Default for FormatSettings {
    fn default() -> Self {
        Self {
            minutes_threshold: 60_000,
            millis_threshold: 10_000,
        }
    }
}

impl FormatSettings {
    pub fn render_options(&mut self, ui: &Ui) {
        let Self {
            minutes_threshold,
            millis_threshold,
        } = self;

        input_seconds(ui, "Minute threshold", minutes_threshold);
        helper(ui, || {
            ui.text("Above how many seconds to display MM:SS format");
        });

        input_seconds(ui, "Millisecond threshold", millis_threshold);
        helper(ui, || {
            ui.text("Below how many seconds to display milliseconds");
            ui.text("MM:SS format always hides milliseconds");
        });
    }
}
