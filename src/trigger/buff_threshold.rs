use crate::{render_util::enum_combo, traits::RenderOptions};
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
    Min(i32),

    // Maximum amount of stacks.
    #[strum(serialize = "Max Stacks")]
    Max(i32),

    // Range of stacks.
    #[strum(serialize = "Range of Stacks")]
    Between(i32, i32),
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
    pub fn is_met(&self, stacks: i32) -> bool {
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
                    ui.input_int("Stacks", required).build();
                }
                Self::Between(min, max) => {
                    ui.input_int("Min Stacks", min).build();
                    ui.input_int("Max Stacks", max).build();
                }
            }
        })
    }
}
