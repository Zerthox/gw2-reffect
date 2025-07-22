use crate::{
    colors::{self, with_alpha},
    elements::text::TextDecoration,
    render::{
        enum_combo, helper, input_color_alpha, input_float_with_format, input_percent, input_pos,
        input_seconds, input_u32,
    },
    trigger::ProgressActive,
};
use const_default::ConstDefault;
use nexus::imgui::{ComboBoxFlags, InputTextFlags, Ui};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IconSettings {
    pub stack_text: StackTextSettings,
    pub duration_text: DurationTextSettings,
    pub duration_bar: DurationBarSettings,
}

impl IconSettings {
    #[inline]
    pub const fn new() -> Self {
        Self {
            stack_text: StackTextSettings::new(),
            duration_text: DurationTextSettings::new(),
            duration_bar: DurationBarSettings::new(),
        }
    }
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

impl StackTextSettings {
    #[inline]
    pub const fn new() -> Self {
        Self {
            threshold: 2,
            scale: 0.5,
            offset: [1.0, 1.0],
            color: with_alpha(colors::WHITE, 0.8),
            decoration: TextDecoration::Shadow,
        }
    }

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

impl ConstDefault for StackTextSettings {
    const DEFAULT: Self = Self::new();
}

impl Default for StackTextSettings {
    fn default() -> Self {
        Self::new()
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
    #[inline]
    pub const fn new() -> Self {
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

impl ConstDefault for DurationTextSettings {
    const DEFAULT: Self = Self::new();
}

impl Default for DurationTextSettings {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DurationBarSettings {
    pub height: f32,
    pub color: [f32; 4],
}

impl DurationBarSettings {
    #[inline]
    pub const fn new() -> Self {
        Self {
            height: 2.0,
            color: colors::GREEN,
        }
    }

    pub fn render_options(&mut self, ui: &Ui) {
        let Self { height, color } = self;
        input_float_with_format("Height", height, 1.0, 10.0, "%.2f", InputTextFlags::empty());
        input_color_alpha(ui, "Color", color);
    }
}

impl ConstDefault for DurationBarSettings {
    const DEFAULT: Self = Self::new();
}

impl Default for DurationBarSettings {
    fn default() -> Self {
        Self::new()
    }
}
