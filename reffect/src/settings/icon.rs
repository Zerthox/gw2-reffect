use crate::{
    elements::text::TextDecoration,
    render::colors::{self, with_alpha},
    render_util::{
        enum_combo, helper, input_color_alpha, input_float_with_format, input_percent, input_pos,
        input_seconds,
    },
};
use nexus::imgui::{ComboBoxFlags, InputTextFlags, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IconSettings {
    pub stack_text: StackTextSettings,
    pub duration_text: DurationTextSettings,
    pub duration_bar: DurationBarSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct StackTextSettings {
    pub scale: f32,
    pub offset: [f32; 2],
    pub color: [f32; 4],
    pub decoration: TextDecoration,
}

impl Default for StackTextSettings {
    fn default() -> Self {
        Self {
            scale: 0.5,
            offset: [1.0, 1.0],
            color: with_alpha(colors::WHITE, 0.8),
            decoration: TextDecoration::Shadow,
        }
    }
}

impl StackTextSettings {
    pub fn render_options(&mut self, ui: &Ui) {
        let Self {
            scale,
            offset,
            color,
            decoration,
        } = self;
        input_percent("Scale", scale);
        input_pos(offset);
        enum_combo(ui, "Decoration", decoration, ComboBoxFlags::empty());
        input_color_alpha(ui, "Color", color);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DurationTextSettings {
    pub max_remain: u32,
    pub scale: f32,
    pub color: [f32; 4],
    pub color_fast: [f32; 4],
    pub color_slow: [f32; 4],
    pub decoration: TextDecoration,
}

impl Default for DurationTextSettings {
    fn default() -> Self {
        Self {
            max_remain: 5000,
            scale: 0.5,
            color: colors::WHITE,
            color_fast: colors::GREEN,
            color_slow: colors::CYAN,
            decoration: TextDecoration::Outline,
        }
    }
}

impl DurationTextSettings {
    pub fn render_options(&mut self, ui: &Ui) {
        let Self {
            max_remain,
            scale,
            color,
            color_fast,
            color_slow,
            decoration,
        } = self;
        input_seconds("Remaining", max_remain);
        helper(ui, || ui.text("Below which remaining time to display"));
        input_percent("Scale", scale);
        enum_combo(ui, "Decoration", decoration, ComboBoxFlags::empty());
        input_color_alpha(ui, "Color", color);
        input_color_alpha(ui, "Color Slow", color_slow);
        input_color_alpha(ui, "Color Fast", color_fast);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DurationBarSettings {
    pub height: f32,
    pub color: [f32; 4],
}

impl Default for DurationBarSettings {
    fn default() -> Self {
        Self {
            height: 2.0,
            color: colors::GREEN,
        }
    }
}

impl DurationBarSettings {
    pub fn render_options(&mut self, ui: &Ui) {
        let Self { height, color } = self;
        input_float_with_format("Height", height, 1.0, 10.0, "%.2f", InputTextFlags::empty());
        input_color_alpha(ui, "Color", color);
    }
}
