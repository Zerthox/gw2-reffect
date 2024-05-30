use super::Trigger;
use crate::{
    action::Action,
    context::{Context, MapCategory},
    render_util::{enum_combo, impl_static_variants, input_u32},
    traits::RenderOptions,
};
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter};

#[derive(Debug, Default, Clone, AsRefStr, EnumIter, Serialize, Deserialize)]
pub enum MapTrigger {
    #[default]
    Any,
    Category(MapCategory), // TODO: multi category trigger?
    Ids(Vec<u32>),
}

impl_static_variants!(MapTrigger);

impl Trigger for MapTrigger {
    fn is_active(&mut self, ctx: &Context) -> bool {
        match self {
            Self::Any => true,
            Self::Category(category) => ctx.map.category == *category,
            Self::Ids(ids) => ids.iter().copied().any(|id| ctx.map.is_on_map(id)),
        }
    }
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
                let mut action = Action::new();
                for (i, id) in ids.iter_mut().enumerate() {
                    let _id = ui.push_id(i as i32);
                    action.set_next_input_size(ui);
                    input_u32(ui, "##id", id, 0, 0);

                    ui.same_line();
                    action.render_buttons(ui, i);

                    ui.same_line();
                    ui.text(format!("Map id {}", i + 1));
                }
                if ui.button("Add") {
                    ids.push(0);
                }

                action.perform(ids);
            }
        }
    }
}
