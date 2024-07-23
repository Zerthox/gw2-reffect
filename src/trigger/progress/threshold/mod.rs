pub mod amount_type;
mod threshold_type;

use super::ProgressActive;
use crate::{
    context::Context,
    context::EditState,
    render_util::{enum_combo, helper, input_u32},
    traits::RenderOptions,
};
use amount_type::AmountType;
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use threshold_type::ThresholdType;
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProgressThreshold {
    /// Threshold type.
    pub threshold_type: ThresholdType,

    /// Amount type.
    #[serde(default)]
    pub amount_type: AmountType,
}

impl ProgressThreshold {
    pub fn is_met(&self, progress_active: &ProgressActive, ctx: &Context) -> bool {
        let progress = match self.amount_type {
            AmountType::Intensity => progress_active.intensity() as f32,
            AmountType::Duration => {
                Self::format_seconds(progress_active.current(ctx.now).unwrap_or_default())
            }
        };

        match self.threshold_type {
            ThresholdType::Always => true,
            ThresholdType::Present => progress > 0.0,
            ThresholdType::Missing => progress == 0.0,
            ThresholdType::Min(required) => progress >= required as f32,
            ThresholdType::Max(required) => progress <= required as f32,
            ThresholdType::Exact(required) => progress == required as f32,
            ThresholdType::Between(min, max) => (min as f32..=max as f32).contains(&progress),
        }
    }
    fn format_seconds(value: u32) -> f32 {
        value as f32 / 1000.0
    }
}

impl RenderOptions for ProgressThreshold {
    fn render_options(&mut self, ui: &Ui, _state: &mut EditState) {
        ui.group(|| {
            enum_combo(
                ui,
                "Threshold",
                &mut self.threshold_type,
                ComboBoxFlags::empty(),
            );
            helper(ui, || ui.text("When to display"));

            let (input_label, input_helper);
            match self.amount_type {
                AmountType::Intensity => {
                    input_label = "Intensity";
                    input_helper = "Intensity of the effect";
                }
                AmountType::Duration => {
                    input_label = "Duration";
                    input_helper = "Duration in seconds";
                }
            }

            match self.threshold_type {
                ThresholdType::Always | ThresholdType::Present | ThresholdType::Missing => {}
                ThresholdType::Min(ref mut required)
                | ThresholdType::Max(ref mut required)
                | ThresholdType::Exact(ref mut required) => {
                    enum_combo(
                        ui,
                        "Amount type",
                        &mut self.amount_type,
                        ComboBoxFlags::empty(),
                    );
                    helper(ui, || ui.text("Type of the amount to filter"));
                    input_u32(ui, input_label, required, 1, 10);
                    helper(ui, || ui.text(input_helper));
                }
                ThresholdType::Between(ref mut min, ref mut max) => {
                    enum_combo(
                        ui,
                        "Amount type",
                        &mut self.amount_type,
                        ComboBoxFlags::empty(),
                    );
                    helper(ui, || ui.text("Type of the amount to filter"));
                    input_u32(
                        ui,
                        format!("Min {}", input_label.to_lowercase()),
                        min,
                        1,
                        10,
                    );
                    helper(ui, || ui.text(input_helper));
                    input_u32(
                        ui,
                        format!("Max {}", input_label.to_lowercase()),
                        max,
                        1,
                        10,
                    );
                    helper(ui, || ui.text(input_helper));
                }
            }
        })
    }
}
