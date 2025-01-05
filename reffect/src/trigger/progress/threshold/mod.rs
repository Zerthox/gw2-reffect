mod amount_type;
mod threshold_type;

pub use self::{amount_type::*, threshold_type::*};

use super::ProgressActive;
use crate::{
    context::Context,
    render::RenderOptions,
    render_util::{enum_combo, helper},
};
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProgressThreshold {
    /// Threshold type.
    pub threshold_type: ThresholdType,

    /// Amount type.
    #[serde(default)] // TODO: move up after migration period
    pub amount_type: AmountType,
}

impl ProgressThreshold {
    pub fn is_met(&self, active: &ProgressActive, ctx: &Context) -> bool {
        let amount = self.amount_type.amount(active, ctx);
        match self.threshold_type {
            ThresholdType::Always => true,
            ThresholdType::Present => active.intensity() > 0, // we use intensity for present checks
            ThresholdType::Missing => active.intensity() == 0,
            ThresholdType::Below(required) => amount <= required,
            ThresholdType::Above(required) => amount >= required,
            ThresholdType::Exact(required) => amount == required,
            ThresholdType::Between(min, max) => (min..=max).contains(&amount),
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

impl fmt::Display for ProgressThreshold {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.threshold_type.no_amount() {
            self.threshold_type.fmt(f)
        } else {
            write!(f, "{} {}", self.amount_type, self.threshold_type)
        }
    }
}

impl RenderOptions for ProgressThreshold {
    fn render_options(&mut self, ui: &Ui, ctx: &Context) {
        enum_combo(
            ui,
            "Threshold",
            &mut self.threshold_type,
            ComboBoxFlags::empty(),
        );
        helper(ui, || ui.text("Threshold required to be met"));

        match &mut self.threshold_type {
            ThresholdType::Always | ThresholdType::Present | ThresholdType::Missing => {}
            ThresholdType::Above(required)
            | ThresholdType::Below(required)
            | ThresholdType::Exact(required) => {
                self.amount_type.render_options(ui, ctx);
                self.amount_type.render_input(ui, "Amount", required);
            }
            ThresholdType::Between(min, max) => {
                self.amount_type.render_options(ui, ctx);
                self.amount_type.render_input(ui, "Min", min);
                self.amount_type.render_input(ui, "Max", max);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde_migrate::Migrate;

    #[test]
    fn migrate() {
        let json = r#"{ "Between": [1, 23] }"#;
        let result = serde_json::from_str::<Migrate<ProgressThreshold, ThresholdType>>(&json);
        assert!(result.is_ok());
        let threshold = result.unwrap().inner;
        assert_eq!(threshold.threshold_type, ThresholdType::Between(1.0, 23.0));
    }
}
