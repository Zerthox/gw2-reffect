use super::{TextSettings, legacy::DurationTextSettingsLegacy};
use crate::{
    colors,
    elements::{Anchor, text::TextDecoration},
    render::{helper, input_color_alpha, input_seconds},
    serde::migrate,
    trigger::ProgressActive,
};
use const_default::ConstDefault;
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DurationTextSettings {
    #[serde(alias = "max_remain")]
    pub threshold_buff: u32,

    pub threshold_ability: u32,

    #[serde(
        flatten,
        deserialize_with = "migrate::<_, _, DurationTextSettingsLegacy>"
    )]
    pub text: TextSettings,

    pub color_fast: [f32; 4],
    pub color_slow: [f32; 4],
}

impl DurationTextSettings {
    #[inline]
    pub const fn new() -> Self {
        Self {
            threshold_buff: 5000,
            threshold_ability: u32::MAX,
            text: TextSettings {
                scale: 0.5,
                anchor: Anchor::Center,
                offset: [0.0, 0.0],
                decoration: TextDecoration::Outline,
                color: colors::WHITE,
            },
            color_fast: colors::GREEN,
            color_slow: colors::CYAN,
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
            Ordering::Equal => self.text.color,
            Ordering::Greater => self.color_fast,
        }
    }

    pub fn render_options(&mut self, ui: &Ui) {
        let Self {
            threshold_buff,
            threshold_ability,
            text,
            color_fast,
            color_slow,
        } = self;

        input_seconds(ui, "Threshold Effect", threshold_buff);
        helper(ui, || {
            ui.text("Below how many remaining seconds to display for effects")
        });

        input_seconds(ui, "Threshold Ability", threshold_ability);
        helper(ui, || {
            ui.text("Below how many remaining seconds to display for abilities")
        });

        text.render_options(ui);
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
