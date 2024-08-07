use super::PartialProps;
use crate::{
    colors,
    context::EditState,
    render_util::{input_color_alpha, input_optional},
    traits::RenderOptions,
};
use nexus::imgui::Ui;
use partial::Partial;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Partial, Serialize, Deserialize)]
#[partial(derive(Debug, Clone, Serialize, Deserialize))]
#[serde(default)]
pub struct IconProps {
    #[serde(alias = "color")]
    pub tint: [f32; 4],
}

impl Default for IconProps {
    fn default() -> Self {
        Self {
            tint: colors::WHITE,
        }
    }
}

impl RenderOptions for IconProps {
    fn render_options(&mut self, ui: &Ui, _state: &mut EditState) {
        let Self { tint } = self;
        input_color_alpha(ui, "Tint", tint);
    }
}

impl PartialProps<IconProps> for Partial<IconProps> {
    fn render_options(&mut self, ui: &Ui, base: &IconProps) {
        let Self { tint } = self;
        input_optional(
            ui,
            "Tint",
            tint,
            || base.tint,
            |tint| input_color_alpha(ui, "Tint", tint),
        );
    }
}
