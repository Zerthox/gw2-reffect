use crate::{
    context::{Context, EditState},
    render_util::{enum_combo, helper, input_float_with_format},
    traits::RenderOptions,
    trigger::ProgressActive,
};
use nexus::imgui::{ComboBoxFlags, InputTextFlags, Ui};
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
}

impl AmountType {
    pub fn amount(&self, active: &ProgressActive, ctx: &Context) -> f32 {
        match self {
            Self::Intensity => active.intensity() as f32,
            Self::Duration => active
                .current(ctx.now)
                .map(|current| current as f32 / 1000.0)
                .unwrap_or(0.0),
            Self::Percent => active.progress_or_default(ctx.now),
        }
    }

    pub fn render_input(&self, ui: &Ui, label: impl Into<String>, value: &mut f32) -> bool {
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
                    *value = value.round();
                }
                helper(ui, || ui.text("Intensity in stacks or resource units"));
                changed
            }
            Self::Duration => {
                let changed = input_float_with_format(
                    label,
                    value,
                    1.0,
                    10.0,
                    "%.3f",
                    InputTextFlags::empty(),
                );
                helper(ui, || ui.text("Intensity in stacks or resource units"));
                changed
            }
            Self::Percent => {
                let changed = input_float_with_format(
                    label,
                    value,
                    1.0,
                    10.0,
                    "%.2f",
                    InputTextFlags::empty(),
                );
                helper(ui, || ui.text("Intensity in stacks or resource units"));
                changed
            }
        }
    }
}

impl RenderOptions<Option<Self>> for AmountType {
    fn render_options(&mut self, ui: &Ui, _state: &mut EditState) -> Option<Self> {
        let result = enum_combo(ui, "Amount type", self, ComboBoxFlags::empty());
        helper(ui, || ui.text("Type of amount to check"));
        result
    }
}
