pub mod legacy;

use super::{check_bitflags, memo::MemoizedTrigger};
use crate::{
    action::Action,
    context::{Context, ContextUpdate, MapCategory},
    render::RenderOptions,
    render_util::{enum_combo_bitflags, helper, input_u32, item_context_menu, map_select},
    serde::bitflags,
};
use enumflags2::BitFlags;
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapTrigger {
    #[serde(with = "bitflags")]
    pub category: BitFlags<MapCategory>,

    #[serde(default = "default_true")]
    pub whitelist: bool,

    #[serde(default)] // TODO: move up after migration end
    pub ids: Vec<u32>,

    #[serde(skip)]
    pub memo: Option<bool>,
}

fn default_true() -> bool {
    true
}

impl MemoizedTrigger for MapTrigger {
    fn needs_update(&self, ctx: &Context) -> bool {
        ctx.has_update(ContextUpdate::Map)
    }

    fn memo(&mut self) -> &mut Option<bool> {
        &mut self.memo
    }

    fn is_active_current(&mut self, ctx: &Context) -> bool {
        check_bitflags(self.category, ctx.map.category)
            && (self.ids.is_empty() || {
                let id_match = self.ids.iter().any(|id| ctx.map.is_on_map(*id));
                if self.whitelist {
                    id_match
                } else {
                    !id_match
                }
            })
    }
}

impl RenderOptions<bool> for MapTrigger {
    fn render_options(&mut self, ui: &Ui, _ctx: &Context) -> bool {
        let _id = ui.push_id("map");

        let mut changed = enum_combo_bitflags(
            ui,
            "Map Category",
            &mut self.category,
            ComboBoxFlags::empty(),
        );

        let mut action = Action::new();
        for (i, id) in self.ids.iter_mut().enumerate() {
            let _id = ui.push_id(i as i32);
            changed |= action.input_with_buttons(ui, i, || input_u32(ui, "##id", id, 0, 0));

            ui.same_line();
            ui.text(format!("Map Id {}", i + 1));

            if i == 0 {
                helper(ui, || {
                    ui.text("Can be found on the wiki, same as in GW2 API")
                });
            }
        }
        if ui.button("Add Map Id") {
            self.ids.push(0);
            changed = true;
        }
        item_context_menu("addctx", || {
            if let Some(maps) = map_select(ui) {
                self.ids.extend(maps.iter().map(|map| map.id));
            }
        });
        helper(ui, || ui.text("Right click to insert commonly used maps"));

        changed |= action.perform(&mut self.ids);

        changed |= ui.checkbox("Use Whitelist", &mut self.whitelist);
        helper(ui, || {
            ui.text("Whether to use map ids as whitelist or blacklist")
        });

        changed
    }
}

impl Default for MapTrigger {
    fn default() -> Self {
        Self {
            category: BitFlags::empty(),
            whitelist: true,
            ids: Vec::new(),
            memo: None,
        }
    }
}
