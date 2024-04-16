use super::Trigger;
use crate::{
    context::{MapCategory, RenderContext},
    render_util::{enum_combo, impl_static_variants, input_u32},
    traits::{Leaf, RenderOptions},
};
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter};

#[derive(Debug, Default, Clone, AsRefStr, EnumIter, Serialize, Deserialize)]
pub enum MapTrigger {
    #[default]
    Any,
    Category(MapCategory),
    Ids(Vec<u32>),
}

impl_static_variants!(MapTrigger);

impl Trigger for MapTrigger {
    fn is_active(&self, ctx: &RenderContext) -> bool {
        match self {
            Self::Any => true,
            Self::Category(category) => ctx.map.category == *category,
            Self::Ids(ids) => ids.iter().copied().any(|id| ctx.map.is_on_map(id)),
        }
    }
}

impl Leaf for MapTrigger {
    fn load(&mut self) {}
}

impl RenderOptions for MapTrigger {
    fn render_options(&mut self, ui: &Ui) {
        enum_combo(ui, "Map", self, ComboBoxFlags::empty());

        match self {
            Self::Any => {}
            Self::Category(category) => {
                enum_combo(ui, "Category", category, ComboBoxFlags::empty());
            }
            Self::Ids(ids) => {
                // TODO: as single text input?
                for (i, id) in ids.iter_mut().enumerate() {
                    input_u32(ui, format!("Id {}", i + 1), id, 0, 0);
                }
                if ui.button("+") {
                    ids.push(0);
                }
                ui.same_line();
                if ui.button("-") {
                    ids.pop();
                }
            }
        }
    }
}
