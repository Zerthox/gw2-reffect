use crate::{
    render_util::{enum_combo, input_u32},
    traits::RenderOptions,
};
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, IntoStaticStr, VariantArray};

#[derive(Debug, Default, Clone, AsRefStr, IntoStaticStr, EnumIter, Serialize, Deserialize)]
pub enum ProgressThreshold {
    /// Always met.
    Always,

    /// Must be present.
    #[default]
    Present,

    /// Must be missing.
    Missing,

    /// Minimum amount.
    #[strum(serialize = "Min amount")]
    Min(u32),

    /// Maximum amount.
    #[strum(serialize = "Max amount")]
    Max(u32),

    /// Range of.
    #[strum(serialize = "Range of")]
    Between(u32, u32),
}

impl VariantArray for ProgressThreshold {
    const VARIANTS: &'static [Self] = &[
        Self::Always,
        Self::Present,
        Self::Missing,
        Self::Min(1),
        Self::Max(1),
        Self::Between(0, 1),
    ];
}

impl ProgressThreshold {
    pub fn is_met(&self, progress: u32) -> bool {
        match *self {
            Self::Always => true,
            Self::Present => progress > 0,
            Self::Missing => progress == 0,
            Self::Min(required) => progress >= required,
            Self::Max(required) => progress <= required,
            Self::Between(min, max) => (min..=max).contains(&progress),
        }
    }
}

impl RenderOptions for ProgressThreshold {
    fn render_options(&mut self, ui: &Ui) {
        ui.group(|| {
            enum_combo(ui, "Threshold", self, ComboBoxFlags::empty());

            match self {
                Self::Always | Self::Present | Self::Missing => {}
                Self::Min(required) | Self::Max(required) => {
                    input_u32(ui, "Amount", required, 1, 10);
                }
                Self::Between(min, max) => {
                    input_u32(ui, "Min amount", min, 1, 10);
                    input_u32(ui, "Max amount", max, 1, 10);
                }
            }
        })
    }
}
