use crate::{
    context::EditState,
    elements::PartialProps,
    render::{colors, RenderOptions},
    render_util::{
        helper, input_color_alpha, input_optional, input_percent_inverse, slider_percent_capped,
    },
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
    pub zoom: f32, // kept as factor to avoid divisions
    pub round: f32,
}

impl Default for IconProps {
    fn default() -> Self {
        Self {
            tint: colors::WHITE,
            zoom: 1.0,
            round: 0.0,
        }
    }
}

impl RenderOptions for IconProps {
    fn render_options(&mut self, ui: &Ui, _state: &mut EditState) {
        let Self { tint, zoom, round } = self;

        input_color_alpha(ui, "Tint", tint);

        input_percent_inverse("Zoom", zoom);
        helper(ui, || ui.text("Icon zoom in percent"));

        slider_percent_capped(ui, "Round", round, 50.0);
        helper(ui, || ui.text("Corner rounding in percent"));
    }
}

impl PartialProps<IconProps> for Partial<IconProps> {
    fn render_options(&mut self, ui: &Ui, base: &IconProps) {
        let Self { tint, zoom, round } = self;
        input_optional(
            ui,
            "Tint",
            tint,
            || base.tint,
            |tint| input_color_alpha(ui, "Tint", tint),
        );
        input_optional(
            ui,
            "Zoom",
            zoom,
            || base.zoom,
            |zoom| input_percent_inverse("Zoom", zoom),
        );
        input_optional(
            ui,
            "Round",
            round,
            || base.round,
            |round| slider_percent_capped(ui, "Round", round, 50.0),
        );
    }
}
