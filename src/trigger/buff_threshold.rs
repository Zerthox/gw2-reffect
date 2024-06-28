use crate::{
    render_util::{enum_combo, input_u32},
    traits::RenderOptions,
};
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, IntoStaticStr, VariantArray};

#[derive(Debug, Default, Clone, AsRefStr, IntoStaticStr, EnumIter, Serialize, Deserialize)]
pub enum BuffThreshold {
    // Must be present.
    #[default]
    Present,

    // Must be missing.
    Missing,

    // Minimum amount of stacks.
    #[strum(serialize = "Min Stacks")]
    Min(u32),

    // Maximum amount of stacks.
    #[strum(serialize = "Max Stacks")]
    Max(u32),

    // Range of stacks.
    #[strum(serialize = "Range of Stacks")]
    Between(u32, u32),
}

impl VariantArray for BuffThreshold {
    const VARIANTS: &'static [Self] = &[
        Self::Present,
        Self::Missing,
        Self::Min(1),
        Self::Max(1),
        Self::Between(0, 1),
    ];
}

impl BuffThreshold {
    pub fn is_met(&self, stacks: u32) -> bool {
        match *self {
            Self::Present => stacks > 0,
            Self::Missing => stacks == 0,
            Self::Min(required) => stacks >= required,
            Self::Max(required) => stacks <= required,
            Self::Between(min, max) => (min..=max).contains(&stacks),
        }
    }
}

impl RenderOptions for BuffThreshold {
    fn render_options(&mut self, ui: &Ui) {
        ui.group(|| {
            enum_combo(ui, "Threshold", self, ComboBoxFlags::empty());

            match self {
                Self::Present | Self::Missing => {}
                Self::Min(required) | Self::Max(required) => {
                    input_u32(ui, "Stacks", required, 1, 10);
                }
                Self::Between(min, max) => {
                    input_u32(ui, "Min Stacks", min, 1, 10);
                    input_u32(ui, "Max Stacks", max, 1, 10);
                }
            }
        })
    }
}
