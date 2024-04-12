use nexus::imgui::{InputTextFlags, Ui};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{Element, RenderState};
use crate::{
    component_wise::ComponentWise,
    context::{EditState, RenderContext},
    util::{ch_width, input_float_with_format, text_label, tree_select},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Common {
    #[serde(skip)]
    pub id: Uuid,

    pub name: String,
    pub pos: [f32; 2],
}

impl Common {
    pub fn render(
        &self,
        ui: &Ui,
        ctx: &RenderContext,
        state: &RenderState,
        children: impl FnOnce(&RenderState),
    ) {
        let edit = ctx.edit.is_active_or_parent(self.id);
        let state = state.with_edit(edit).with_offset(self.pos);
        children(&state);

        if ctx.edit.is_active(self.id) {
            self.render_edit_indicator(ui);
        }
    }

    fn render_edit_indicator(&self, ui: &Ui) {
        const SIZE: f32 = 5.0;
        const HALF_SIZE: f32 = SIZE / 2.0;
        const OFFSET: [f32; 2] = [HALF_SIZE, HALF_SIZE];
        const COLOR: [f32; 4] = [1.0, 0.0, 0.0, 0.8];

        let start = self.pos.sub(OFFSET);
        let end = self.pos.add(OFFSET);
        ui.get_window_draw_list()
            .add_rect(start, end, COLOR)
            .filled(true)
            .build();

        ui.set_cursor_screen_pos(self.pos.add([HALF_SIZE + 1.0, 0.0]));
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
    ) -> bool {
        let id = self.id.simple().to_string();
        let label = format!("{kind}: {}##{id}", self.name);
        let is_selected = state.is_active(self.id);
        if tree_select(ui, id, label, is_selected, children.is_empty(), || {
            for child in children {
                child.render_select_tree(ui, state);
            }
        }) {
            if is_selected {
                state.selected = Uuid::nil();
            } else {
                state.selected = self.id;
            }
        }
        state.selected == self.id
    }

    /// Renders the common options.
    pub fn render_options(&mut self, ui: &Ui) {
        text_label(ui, "Name");
        ui.input_text("##name", &mut self.name).build();

        let [x, y] = &mut self.pos;
        let size = ch_width(ui, 12);
        text_label(ui, "Position x");
        ui.set_next_item_width(size);
        input_float_with_format("##posx", x, 1.0, 10.0, "%0.f", InputTextFlags::empty());
        text_label(ui, "Position x");
        ui.set_next_item_width(size);
        input_float_with_format("##posy", y, 1.0, 10.0, "%0.f", InputTextFlags::empty());
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
