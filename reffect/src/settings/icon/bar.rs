use crate::{
    colors,
    render::{input_color_alpha, input_float_with_format},
};
use const_default::ConstDefault;
use nexus::imgui::{InputTextFlags, Ui};
use serde::{Deserialize, Serialize};

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
