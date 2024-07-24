mod amount_type;
mod threshold_type;

pub use self::{amount_type::*, threshold_type::*};

use super::ProgressActive;
use crate::{
    context::{Context, EditState},
    render_util::{enum_combo, helper, input_seconds, input_u32},
    traits::RenderOptions,
};
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ProgressThreshold {
    /// Threshold type.
    pub threshold_type: ThresholdType,

    /// Amount type.
    pub amount_type: AmountType,
}

impl ProgressThreshold {
    pub fn is_met(&self, active: &ProgressActive, ctx: &Context) -> bool {
        let progress = match self.amount_type {
            AmountType::Intensity => active.intensity(),
            AmountType::Duration => active.current(ctx.now).unwrap_or(0),
        };

        match self.threshold_type {
            ThresholdType::Always => true,
            ThresholdType::Present => active.intensity() > 0,
            ThresholdType::Missing => active.intensity() == 0,
            ThresholdType::Min(required) => progress >= required,
            ThresholdType::Max(required) => progress <= required,
            ThresholdType::Exact(required) => progress == required,
            ThresholdType::Between(min, max) => (min..=max).contains(&progress),
        }
    }
}

impl From<ThresholdType> for ProgressThreshold {
    fn from(threshold_type: ThresholdType) -> Self {
        Self {
            threshold_type,
            amount_type: AmountType::Intensity,
        }
    }
}

impl RenderOptions for ProgressThreshold {
    fn render_options(&mut self, ui: &Ui, state: &mut EditState) {
        enum_combo(
            ui,
            "Threshold",
            &mut self.threshold_type,
            ComboBoxFlags::empty(),
        );
        helper(ui, || ui.text("When to display"));

        match &mut self.threshold_type {
            ThresholdType::Always | ThresholdType::Present | ThresholdType::Missing => {}
            ThresholdType::Min(required)
            | ThresholdType::Max(required)
            | ThresholdType::Exact(required) => {
                if let Some(prev) = self.amount_type.render_options(ui, state) {
                    match (prev, self.amount_type) {
                        (AmountType::Intensity, AmountType::Duration) => *required *= 1000,
                        (AmountType::Duration, AmountType::Intensity) => *required /= 1000,
                        _ => {}
                    }
                }

                match self.amount_type {
                    AmountType::Intensity => {
                        input_u32(ui, "Intensity", required, 1, 10);
                    }
                    AmountType::Duration => {
                        input_seconds("Duration", required);
                        helper(ui, || ui.text("Duration in seconds"));
                    }
                }
            }
            ThresholdType::Between(min, max) => {
                if let Some(prev) = self.amount_type.render_options(ui, state) {
                    match (prev, self.amount_type) {
                        (AmountType::Intensity, AmountType::Duration) => {
                            *min *= 1000;
                            *max *= 1000;
                        }
                        (AmountType::Duration, AmountType::Intensity) => {
                            *min /= 1000;
                            *max /= 1000;
                        }
                        _ => {}
                    }
                }

                match self.amount_type {
                    AmountType::Intensity => {
                        input_u32(ui, "Min intensity", min, 1, 10);
                        input_u32(ui, "Max intensity", max, 1, 10);
                    }
                    AmountType::Duration => {
                        input_seconds("Min duration", min);
                        helper(ui, || ui.text("Duration in seconds"));
                        input_seconds("Max duration", max);
                    }
                };
            }
        }
    }
}
