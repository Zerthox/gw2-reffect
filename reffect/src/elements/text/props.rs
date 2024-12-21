use super::TextDecoration;
use crate::{
    context::Context,
    elements::PartialProps,
    render::{colors, RenderOptions},
    render_util::{input_color_alpha, input_optional, input_percent},
};
use nexus::imgui::Ui;
use partial::Partial;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Partial, Serialize, Deserialize)]
#[partial(derive(Debug, Clone, Serialize, Deserialize))]
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
            color: colors::WHITE,
            decoration: TextDecoration::default(),
        }
    }
}

impl RenderOptions for TextProps {
    fn render_options(&mut self, ui: &Ui, _ctx: &Context) {
        let Self {
            scale,
            color,
            decoration,
        } = self;
        input_percent("Scale", scale);
        input_color_alpha(ui, "Color", color);
        decoration.render_select(ui);
    }
}

impl PartialProps<TextProps> for Partial<TextProps> {
    fn render_options(&mut self, ui: &Ui, base: &TextProps) {
        let Self {
            scale,
            color,
            decoration,
        } = self;
        input_optional(
            ui,
            "Scale",
            scale,
            || base.scale,
            |scale| input_percent("Scale", scale),
        );
        input_optional(
            ui,
            "Color",
            color,
            || base.color,
            |color| input_color_alpha(ui, "Color", color),
        );
        input_optional(
            ui,
            "Decoration",
            decoration,
            || base.decoration,
            |decoration| decoration.render_select(ui),
        );
    }
}
