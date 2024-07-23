use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, IntoStaticStr, VariantArray};

use crate::{
    context::EditState,
    render_util::{enum_combo, helper},
    traits::RenderOptions,
};

#[derive(
    Debug, Default, Clone, AsRefStr, IntoStaticStr, EnumIter, Serialize, Deserialize, VariantArray,
)]
pub enum AmountType {
    // Intensity.
    #[default]
    Intensity,

    // Duration.
    Duration,
}

impl RenderOptions for AmountType {
    fn render_options(&mut self, ui: &Ui, _state: &mut EditState) {
        enum_combo(ui, "Amount type", self, ComboBoxFlags::empty());
        helper(ui, || ui.text("Type of amount to check"));
    }
}
