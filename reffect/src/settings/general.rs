use super::icon::IconSettings;
use crate::render::{LoadedFont, helper, input_seconds};
use const_default::ConstDefault;
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

impl GeneralSettings {
    pub const fn new() -> Self {
        Self {
            save_on_unload: true,
            use_game_icons: false,
            format: FormatSettings::new(),
            font: LoadedFont::empty(),
            icon: IconSettings::new(),
        }
    }

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

impl ConstDefault for GeneralSettings {
    const DEFAULT: Self = Self::new();
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FormatSettings {
    pub minutes_threshold: u32,
    pub millis_threshold: u32,
}

impl FormatSettings {
    pub const fn new() -> Self {
        Self {
            minutes_threshold: 60_000,
            millis_threshold: 10_000,
        }
    }

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

impl ConstDefault for FormatSettings {
    const DEFAULT: Self = Self::new();
}

impl Default for FormatSettings {
    fn default() -> Self {
        Self::new()
    }
}
