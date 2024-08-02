use super::Trigger;
use crate::{
    action::Action,
    context::{Context, EditState, MapCategory},
    render_util::{
        enum_combo, helper, impl_static_variants, input_u32, item_context_menu, map_select,
    },
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

impl RenderOptions<bool> for MapTrigger {
    fn render_options(&mut self, ui: &Ui, _state: &mut EditState) -> bool {
        let mut changed = enum_combo(ui, "Map", self, ComboBoxFlags::empty()).is_some();

        match self {
            Self::Any => {}
            Self::Category(category) => {
                changed |= enum_combo(ui, "Category", category, ComboBoxFlags::empty()).is_some();
            }
            Self::Ids(ids) => {
                let mut action = Action::new();
                for (i, id) in ids.iter_mut().enumerate() {
                    let _id = ui.push_id(i as i32);
                    changed |= action.input_with_buttons(ui, i, || input_u32(ui, "##id", id, 0, 0));

                    ui.same_line();
                    ui.text(format!("Map Id {}", i + 1));

                    if i == 0 {
                        helper(ui, || {
                            ui.text("Same as in GW2 API");
                            ui.text("Can be found on the wiki");
                        });
                    }
                }
                if ui.button("Add Map") {
                    ids.push(0);
                    changed = true;
                }
                item_context_menu("addctx", || {
                    if let Some(maps) = map_select(ui) {
                        ids.extend(maps.iter().map(|map| map.id));
                    }
                });
                helper(ui, || ui.text("Right click to insert commonly used maps"));

                changed |= action.perform(ids);
            }
        }

        changed
    }
}
