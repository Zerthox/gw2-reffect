use crate::{
    action::DynAction,
    colors,
    elements::PartialProps,
    render::{
        helper, input_color_alpha, input_optional, input_percent_inverse,
        input_positive_with_format, slider_percent_capped,
    },
    render_copy_field,
};
use nexus::imgui::{InputTextFlags, Ui};
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

    pub border_size: f32,
    pub border_color: [f32; 4],
}

impl Default for IconProps {
    fn default() -> Self {
        Self {
            tint: colors::WHITE,
            zoom: 1.0,
            round: 0.0,
            border_size: 0.0,
            border_color: colors::BLACK,
        }
    }
}

impl IconProps {
    pub fn render_options(&mut self, ui: &Ui) -> DynAction<Self> {
        let Self {
            tint,
            zoom,
            round,
            border_size,
            border_color,
        } = self;

        let mut action = DynAction::<Self>::empty();

        input_color_alpha(ui, "Tint", tint);
        render_copy_field!(action, ui, *tint);

        input_percent_inverse("Zoom", zoom);
        render_copy_field!(action, ui, *zoom);
        helper(ui, || ui.text("Icon zoom in percent"));

        slider_percent_capped(ui, "Round", round, 50.0);
        render_copy_field!(action, ui, *round);
        helper(ui, || ui.text("Corner rounding in percent"));

        input_positive_with_format(
            "Border size",
            border_size,
            1.0,
            10.0,
            "%.1f",
            InputTextFlags::empty(),
        );
        render_copy_field!(action, ui, *border_size);

        input_color_alpha(ui, "Border color", border_color);
        render_copy_field!(action, ui, *border_color);

        action
    }
}

impl PartialProps<IconProps> for Partial<IconProps> {
    fn render_options(&mut self, ui: &Ui, base: &IconProps) {
        let Self {
            tint,
            zoom,
            round,
            border_size,
            border_color,
        } = self;
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

        input_optional(
            ui,
            "Border size",
            border_size,
            || base.border_size,
            |size| {
                input_positive_with_format(
                    "Border size",
                    size,
                    1.0,
                    10.0,
                    "%.1f",
                    InputTextFlags::empty(),
                )
            },
        );
        input_optional(
            ui,
            "Border color",
            border_color,
            || base.border_color,
            |color| input_color_alpha(ui, "Border color", color),
        );
    }
}
