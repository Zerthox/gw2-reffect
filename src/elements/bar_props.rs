use super::PartialProps;
use crate::{
    colors,
    context::EditState,
    render_util::{
        input_color_alpha, input_optional, input_percent, input_percent_inverse,
        input_positive_with_format,
    },
    traits::RenderOptions,
};
use nexus::imgui::{InputTextFlags, Ui};
use partial::Partial;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Partial, Serialize, Deserialize)]
#[partial(derive(Debug, Clone, Serialize, Deserialize))]
#[serde(default)]
pub struct BarProps {
    pub lower_bound: f32,
    pub progress_factor: f32,

    pub fill: [f32; 4],
    pub background: [f32; 4],

    pub border_size: f32,
    pub border_color: [f32; 4],

    pub tick_size: f32,
    pub tick_color: [f32; 4],
}

impl Default for BarProps {
    fn default() -> Self {
        Self {
            lower_bound: 0.0,
            progress_factor: 1.0,
            fill: colors::GREEN,
            background: colors::TRANSPARENT,
            border_size: 1.0,
            border_color: colors::BLACK,
            tick_size: 1.0,
            tick_color: colors::BLACK,
        }
    }
}

impl RenderOptions for BarProps {
    fn render_options(&mut self, _ui: &Ui, _state: &mut EditState) {
        // rendered by bar
    }
}

impl PartialProps<BarProps> for Partial<BarProps> {
    fn render_options(&mut self, ui: &Ui, base: &BarProps) {
        let Self {
            lower_bound,
            progress_factor,
            fill,
            background,
            border_size,
            border_color,
            tick_size,
            tick_color,
        } = self;

        input_optional(
            ui,
            "Lower bound",
            lower_bound,
            || base.lower_bound,
            |bound| input_percent("Lower bound", bound),
        );
        input_optional(
            ui,
            "Upper bound",
            progress_factor,
            || base.progress_factor,
            |bound| input_percent_inverse("Upper bound", bound),
        );

        input_optional(
            ui,
            "Fill",
            fill,
            || base.fill,
            |color| input_color_alpha(ui, "Fill", color),
        );
        input_optional(
            ui,
            "Background",
            background,
            || base.background,
            |color| input_color_alpha(ui, "Background", color),
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

        input_optional(
            ui,
            "Tick size",
            tick_size,
            || base.tick_size,
            |size| {
                input_positive_with_format(
                    "Tick size",
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
            "Tick color",
            tick_color,
            || base.tick_color,
            |color| input_color_alpha(ui, "Tick color", color),
        );
    }
}
