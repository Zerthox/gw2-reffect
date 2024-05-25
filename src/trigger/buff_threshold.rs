use crate::{
    render_util::{enum_combo, EnumStaticVariants},
    traits::RenderOptions,
};
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, IntoStaticStr};

// TODO: none variant more intuitive than max 0?
// TODO: between allowing range of stacks?

#[derive(Debug, Clone, AsRefStr, IntoStaticStr, EnumIter, Serialize, Deserialize)]
pub enum BuffThreshold {
    // Minimum amount of stacks.
    Min(i32),

    // Maximum amount of stacks.
    Max(i32),
}

impl EnumStaticVariants for BuffThreshold {
    fn static_variants() -> &'static [Self] {
        &[Self::Min(1), Self::Max(1)]
    }
}

impl Default for BuffThreshold {
    fn default() -> Self {
        Self::Min(1)
    }
}

impl BuffThreshold {
    pub fn is_met(&self, stacks: i32) -> bool {
        match *self {
            Self::Min(required) => stacks >= required,
            Self::Max(required) => stacks <= required,
        }
    }
}

impl RenderOptions for BuffThreshold {
    fn render_options(&mut self, ui: &Ui) {
        ui.group(|| {
            enum_combo(ui, "Stacks", self, ComboBoxFlags::empty());

            match self {
                Self::Min(required) | Self::Max(required) => {
                    ui.input_int("Count", required).build();
                }
            }
        })
    }
}
