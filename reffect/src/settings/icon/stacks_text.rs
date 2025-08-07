use super::TextSettings;
use crate::{
    colors::{self, with_alpha},
    elements::{Anchor, text::TextDecoration},
    render::{helper, input_u32},
};
use const_default::ConstDefault;
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct StackTextSettings {
    pub threshold: u32,

    #[serde(flatten)]
    pub text: TextSettings,
}

impl StackTextSettings {
    #[inline]
    pub const fn new() -> Self {
        Self {
            threshold: 2,
            text: TextSettings {
                scale: 0.5,
                anchor: Anchor::BottomRight,
                offset: [-1.0, -1.0],
                decoration: TextDecoration::Shadow,
                color: with_alpha(colors::WHITE, 0.8),
            },
        }
    }

    pub fn render_options(&mut self, ui: &Ui) {
        let Self { threshold, text } = self;

        input_u32(ui, "Threshold", threshold, 1, 10);
        helper(ui, || ui.text("Minimum stacks to display"));

        text.render_options(ui);
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
