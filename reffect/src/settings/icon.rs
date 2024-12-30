use std::cmp::Ordering;

use crate::{
    elements::text::TextDecoration,
    render::colors::{self, with_alpha},
    render_util::{
        enum_combo, helper, input_color_alpha, input_float_with_format, input_percent, input_pos,
        input_seconds, input_u32,
    },
    trigger::ProgressActive,
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
    pub threshold: u32,
    pub scale: f32,
    pub offset: [f32; 2],
    pub color: [f32; 4],
    pub decoration: TextDecoration,
}

impl Default for StackTextSettings {
    fn default() -> Self {
        Self {
            threshold: 2,
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
            threshold,
            scale,
            offset,
            color,
            decoration,
        } = self;

        input_u32(ui, "Threshold", threshold, 1, 10);
        helper(ui, || ui.text("Minimum stacks to display"));

        input_percent("Scale", scale);
        input_pos(offset);
        enum_combo(ui, "Decoration", decoration, ComboBoxFlags::empty());
        input_color_alpha(ui, "Color", color);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DurationTextSettings {
    #[serde(alias = "max_remain")]
    pub threshold_buff: u32,

    pub threshold_ability: u32,

    pub scale: f32,
    pub color: [f32; 4],
    pub color_fast: [f32; 4],
    pub color_slow: [f32; 4],
    pub decoration: TextDecoration,
}

impl DurationTextSettings {
    pub fn threshold(&self, active: &ProgressActive) -> u32 {
        match active {
            ProgressActive::Fixed { .. } => u32::MAX,
            ProgressActive::Buff { .. } => self.threshold_buff,
            ProgressActive::Ability { .. } => self.threshold_ability,
        }
    }

    pub fn color(&self, rate: f32) -> [f32; 4] {
        match rate.total_cmp(&1.0) {
            Ordering::Less => self.color_slow,
            Ordering::Equal => self.color,
            Ordering::Greater => self.color_fast,
        }
    }
}

impl Default for DurationTextSettings {
    fn default() -> Self {
        Self {
            threshold_buff: 5000,
            threshold_ability: u32::MAX,
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
            threshold_buff,
            threshold_ability,
            scale,
            color,
            color_fast,
            color_slow,
            decoration,
        } = self;

        input_seconds(ui, "Threshold Effect", threshold_buff);
        helper(ui, || {
            ui.text("Below how many remaining seconds to display for effects")
        });

        input_seconds(ui, "Threshold Ability", threshold_ability);
        helper(ui, || {
            ui.text("Below how many remaining seconds to display for abilities")
        });

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
