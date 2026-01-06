use crate::{
    colors,
    colors::Color,
    elements::PartialProps,
    render::{input_color_alpha, input_optional, input_percent, input_positive_with_format},
};
use const_default::ConstDefault;
use nexus::imgui::{InputTextFlags, Ui};
use partial::Partial;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Partial, Serialize, Deserialize)]
#[partial(derive(Debug, Clone, Serialize, Deserialize))]
#[cfg_attr(
    feature = "schema",
    derive(schemars::JsonSchema),
    partial(derive(schemars::JsonSchema))
)]
#[serde(default)]
pub struct BarProps {
    /// Lower progress bound.
    pub lower_bound: f32,

    /// Upper progress bound.
    pub upper_bound: f32,

    /// Fill color.
    pub fill: Color,

    /// Background color.
    pub background: Color,

    /// Border size.
    pub border_size: f32,

    /// Border color.
    pub border_color: Color,

    /// Tick thickness.
    pub tick_size: f32,

    /// Tick color.
    pub tick_color: Color,
}

impl ConstDefault for BarProps {
    const DEFAULT: Self = Self {
        lower_bound: 0.0,
        upper_bound: 1.0,
        fill: colors::GREEN,
        background: colors::TRANSPARENT,
        border_size: 1.0,
        border_color: colors::BLACK,
        tick_size: 1.0,
        tick_color: colors::BLACK,
    };
}

impl Default for BarProps {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl PartialProps<BarProps> for Partial<BarProps> {
    fn render_options(&mut self, ui: &Ui, base: &BarProps) {
        let Self {
            lower_bound,
            upper_bound,
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
            upper_bound,
            || base.upper_bound,
            |bound| input_percent("Upper bound", bound),
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
