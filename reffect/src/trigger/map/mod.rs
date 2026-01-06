pub mod legacy;

use super::TriggerMode;
use crate::{
    action::Action,
    context::{Context, MapCategory},
    render::{enum_combo_bitflags, helper, input_u32, item_context_menu, map_select},
    serde::bitflags,
    trigger::MemoizedTrigger,
};
use const_default::ConstDefault;
use enumflags2::BitFlags;
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct MapTrigger {
    /// Map category.
    #[serde(with = "bitflags")]
    #[cfg_attr(feature = "schema", schemars(with = "bitflags::Schema<MapCategory>"))]
    pub category: BitFlags<MapCategory>,

    /// Whether to use as whitelist or blacklist.
    #[serde(default = "default_true")]
    pub whitelist: bool,

    /// Map ids.
    #[serde(default)] // TODO: move up after migration end
    pub ids: Vec<u32>,

    #[serde(skip)]
    active: bool,
}

fn default_true() -> bool {
    true
}

impl ConstDefault for MapTrigger {
    const DEFAULT: Self = Self {
        category: BitFlags::EMPTY,
        whitelist: true,
        ids: Vec::new(),
        active: true,
    };
}

impl Default for MapTrigger {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl MapTrigger {
    pub fn non_competitive() -> Self {
        Self {
            category: MapCategory::non_competitive(),
            ..Self::default()
        }
    }
}

impl MemoizedTrigger for MapTrigger {
    fn memoized_state(&mut self) -> &mut bool {
        &mut self.active
    }

    fn needs_update(&self, _ctx: &Context) -> bool {
        false
    }

    fn resolve_active(&mut self, ctx: &Context) -> bool {
        TriggerMode::Any.check_flags(self.category, ctx.map.category)
            && (self.ids.is_empty() || {
                let id_match = self.ids.iter().any(|id| ctx.map.is_on_map(*id));
                if self.whitelist { id_match } else { !id_match }
            })
    }
}

impl MapTrigger {
    pub fn render_options(&mut self, ui: &Ui, ctx: &Context) -> bool {
        let _id = ui.push_id("map");
        let mut changed = false;

        changed |= enum_combo_bitflags(
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
        if ui.button("Add Map") {
            self.ids.push(0);
            changed = true;
        }
        item_context_menu("mapaddctx", || {
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

        if changed {
            // ensure fresh state after changed
            self.update(ctx);
        }

        changed
    }
}
