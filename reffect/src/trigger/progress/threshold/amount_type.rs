use crate::{
    context::Context,
    render::{enum_combo, helper, input_float_with_format},
    trigger::{ProgressActive, ProgressValue},
};
use nexus::imgui::{ComboBoxFlags, InputTextFlags, Slider, SliderFlags, Ui};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, IntoStaticStr, VariantArray};

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Display,
    AsRefStr,
    IntoStaticStr,
    EnumIter,
    Serialize,
    Deserialize,
    VariantArray,
)]
pub enum AmountType {
    /// Intensity.
    #[default]
    Intensity,

    /// Duration.
    Duration,

    /// Progress percent.
    #[strum(serialize = "Progress %")]
    Percent,

    #[strum(serialize = "Secondary Duration")]
    SecondaryDuration,

    #[strum(serialize = "Secondary Progress %")]
    SecondaryPercent,
}

impl AmountType {
    pub fn amount(&self, active: &ProgressActive, ctx: &Context) -> f32 {
        match self {
            Self::Intensity => active.intensity() as f32,
            Self::Duration => active
                .current(ProgressValue::Primary, ctx.now)
                .map(|current| current / 1000.0)
                .unwrap_or(f32::INFINITY),
            Self::Percent => 100.0 * active.progress_or_default(ProgressValue::Primary, ctx.now),
            Self::SecondaryDuration => active
                .current(ProgressValue::Secondary, ctx.now)
                .map(|current| current / 1000.0)
                .unwrap_or(f32::INFINITY),
            &Self::SecondaryPercent => {
                100.0 * active.progress_or_default(ProgressValue::Secondary, ctx.now)
            }
        }
    }

    pub fn render_input(
        &self,
        ui: &Ui,
        label: impl Into<String> + AsRef<str>,
        value: &mut f32,
    ) -> bool {
        match self {
            Self::Intensity => {
                let changed = input_float_with_format(
                    label,
                    value,
                    1.0,
                    10.0,
                    "%.0f",
                    InputTextFlags::empty(),
                );
                if changed {
                    *value = value.round_ties_even();
                }
                helper(ui, || {
                    ui.text("Intensity in stacks or resource units");
                    ui.text("Range includes the value itself");
                });
                changed
            }
            Self::Duration | Self::SecondaryDuration => {
                let changed = input_float_with_format(
                    label,
                    value,
                    1.0,
                    10.0,
                    "%.3f",
                    InputTextFlags::empty(),
                );
                helper(ui, || {
                    ui.text("Duration in seconds");
                    ui.text("Range includes the value itself");
                });
                changed
            }
            Self::Percent | Self::SecondaryPercent => {
                let changed = Slider::new(label, 0.0, 100.0)
                    .flags(SliderFlags::ALWAYS_CLAMP)
                    .display_format("%.2f")
                    .build(ui, value);
                helper(ui, || {
                    ui.text("Progress/duration in percent");
                    ui.text("Range includes the value itself");
                    ui.text("Ctrl+click to type a number");
                });
                changed
            }
        }
    }
}

impl AmountType {
    pub fn render_options(&mut self, ui: &Ui) -> Option<Self> {
        let result = enum_combo(ui, "Amount type", self, ComboBoxFlags::empty());
        helper(ui, || ui.text("Type of amount to check"));
        result
    }
}
