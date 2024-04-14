use super::{Element, ElementType, RenderState};
use crate::{
    component_wise::ComponentWise,
    context::{EditState, RenderContext},
    render_util::{input_float_with_format, item_context_menu, tree_select_custom},
    traits::RenderOptions,
};
use nexus::imgui::{InputTextFlags, MenuItem, Ui};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Common {
    #[serde(skip)]
    pub id: Uuid,

    pub name: String,
    pub pos: [f32; 2],
}

impl Common {
    pub fn id_string(&self) -> String {
        self.id.simple().to_string()
    }

    pub fn render(
        &self,
        ui: &Ui,
        ctx: &RenderContext,
        state: &RenderState,
        children: impl FnOnce(&RenderState),
    ) {
        let mut state = state.with_offset(self.pos);
        state.name = &self.name;
        children(&state);

        if ctx.edit.is_active(self.id) {
            self.render_edit_indicator(ui, &state);
        }
    }

    fn render_edit_indicator(&self, ui: &Ui, state: &RenderState) {
        const SIZE: f32 = 5.0;
        const HALF_SIZE: f32 = 0.5 * SIZE;
        const OFFSET: [f32; 2] = [HALF_SIZE, HALF_SIZE];
        const COLOR: [f32; 4] = [1.0, 0.0, 0.0, 0.8];

        let start = state.pos.sub(OFFSET);
        let end = state.pos.add(OFFSET);
        ui.get_window_draw_list()
            .add_rect(start, end, COLOR)
            .filled(true)
            .build();

        ui.set_cursor_screen_pos(state.pos.add([HALF_SIZE + 1.0, 0.0]));
        ui.text_colored(COLOR, &self.name);
    }

    /// Renders the select tree.
    /// Returns `true` if a child is selected.
    pub fn render_select_tree(
        &self,
        ui: &Ui,
        state: &mut EditState,
        kind: &str,
        children: &mut [Element],
    ) {
        let id = self.id_string();
        let active = state.is_active(self.id);
        let changed = tree_select_custom(
            ui,
            &id,
            active,
            children.is_empty(),
            || {
                ui.same_line();
                ui.text_disabled(kind);
                ui.same_line();
                ui.text(&self.name);
            },
            || {
                for child in children {
                    child.render_select_tree(ui, state);
                }
            },
        );
        if changed {
            state.select(self.id);
        }

        item_context_menu(&id, || {
            ui.menu("Create", || {
                for element in ElementType::iter() {
                    let name = element.as_ref();
                    if MenuItem::new(name).build(ui) {
                        log::debug!("Creating new {name}");
                        // TODO: push element
                    }
                }
            });
            if MenuItem::new("Cut").build(ui) {
                log::debug!("Cutting {kind}");
                // TODO: remove from parent (indicate via return) & save in clipboard
            }
            if MenuItem::new("Copy").build(ui) {
                log::debug!("Copying {kind}");
                // TODO: create copy & save in clipboard
            }
            if MenuItem::new("Paste").build(ui) {
                log::debug!("Pasting unknown");
                // TODO: append element from clipboard
            }
            if MenuItem::new("Delete").build(ui) {
                log::debug!("Deleting {kind}");
                // TODO: remove from parent (indicate via return)
            }
        });
    }

    pub fn render_debug(&mut self, ui: &Ui) {
        ui.text("Id:");
        ui.same_line();
        ui.text_disabled(self.id_string());
    }
}

impl RenderOptions for Common {
    fn render_options(&mut self, ui: &Ui) {
        ui.input_text("Name", &mut self.name).build();

        let [x, y] = &mut self.pos;
        input_float_with_format("Position x", x, 1.0, 10.0, "%.2f", InputTextFlags::empty());
        input_float_with_format("Position y", y, 1.0, 10.0, "%.2f", InputTextFlags::empty());
    }
}

impl Default for Common {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Unnamed".into(),
            pos: [0.0, 0.0],
        }
    }
}

/// Helper to generate options render.
macro_rules! render_or_children {
    ( $self:ident, $ui:expr, $state:expr ) => {
        if $state.is_active($self.common.id) {
            $self.render_options($ui);
            true
        } else {
            $self
                .children()
                .iter_mut()
                .any(|child| child.try_render_options($ui, $state))
        }
    };
}

pub(crate) use render_or_children;
