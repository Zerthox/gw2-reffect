use super::Trigger;
use crate::{
    context::{MapCategory, RenderContext},
    elements::HasOptions,
    util::{enum_combo, impl_static_variants},
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

impl HasOptions for MapTrigger {
    fn render_options(&mut self, ui: &Ui) {
        enum_combo(ui, "Map", self, ComboBoxFlags::empty());
    }
}
