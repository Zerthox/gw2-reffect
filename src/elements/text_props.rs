use crate::{
    context::EditState,
    render_util::{input_color_alpha, input_percent},
    traits::RenderOptions,
};

use super::TextDecoration;
use fields::{AllFields, Field, Fields};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;

#[derive(Debug, Clone, Fields, AllFields, Serialize, Deserialize)]
#[fields(derive(Debug, Clone, AsRefStr, Serialize, Deserialize))]
#[serde(default)]
pub struct TextProps {
    #[serde(alias = "size")]
    pub scale: f32,
    pub color: [f32; 4],
    pub decoration: TextDecoration,
}

impl Default for TextProps {
    fn default() -> Self {
        Self {
            scale: 1.0,
            color: [1.0, 1.0, 1.0, 1.0],
            decoration: TextDecoration::default(),
        }
    }
}

impl RenderOptions for TextProps {
    fn render_options(&mut self, ui: &Ui, _state: &mut EditState) {
        input_percent("Scale", &mut self.scale);
        input_color_alpha(ui, "Color", &mut self.color);
        self.decoration.render_select(ui);
    }
}

impl Default for Field<TextProps> {
    fn default() -> Self {
        Self::Scale(1.0)
    }
}

impl RenderOptions for Field<TextProps> {
    fn render_options(&mut self, ui: &Ui, _state: &mut EditState) {
        match self {
            Self::Scale(scale) => {
                input_percent("Scale", scale);
            }
            Self::Color(color) => {
                input_color_alpha(ui, "Color", color);
            }
            Self::Decoration(decoration) => {
                decoration.render_select(ui);
            }
        }
    }
}
