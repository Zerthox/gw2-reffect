use super::{Element, ElementType, RenderState};
use crate::{
    action::{Action, ChildAction},
    component_wise::ComponentWise,
    context::{EditState, RenderContext},
    render_util::{
        input_float_with_format, item_context_menu, tree_select_custom, EnumStaticVariants,
    },
    traits::RenderOptions,
};
use nexus::imgui::{InputTextFlags, MenuItem, Ui};
use serde::{Deserialize, Serialize};
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
        mut children: Option<&mut Vec<Element>>,
    ) -> Action {
        let id = self.id_string();
        let active = state.is_active(self.id);
        let leaf = children
            .as_ref()
            .map(|children| children.is_empty())
            .unwrap_or(true);
        let changed = tree_select_custom(
            ui,
            &id,
            active,
            leaf,
            || {
                ui.same_line();
                ui.text_disabled(kind);
                ui.same_line();
                ui.text(&self.name);
            },
            || {
                if let Some(children) = children.as_mut() {
                    let mut action = ChildAction::new();
                    for (i, child) in children.iter_mut().enumerate() {
                        action.or(i, child.render_select_tree(ui, state));
                    }
                    action.perform(state, children);
                }
            },
        );
        if changed {
            state.select(self.id);
        }

        let mut action = Action::None;
        item_context_menu(&id, || {
            action = self.render_context_menu(ui, state, children);
        });
        action
    }

    fn render_context_menu(
        &self,
        ui: &Ui,
        state: &mut EditState,
        mut children: Option<&mut Vec<Element>>,
    ) -> Action {
        let mut action = Action::None;
        let has_children = children.is_some();

        ui.menu_with_enabled("Create", has_children, || {
            for kind in ElementType::static_variants() {
                let name = kind.as_ref();
                if MenuItem::new(name).build(ui) {
                    children
                        .as_mut()
                        .expect("create with no children")
                        .push(Element::of_type(kind.clone()));
                }
            }
        });
        if MenuItem::new("Cut").build(ui) {
            action = Action::Cut;
        }
        if MenuItem::new("Copy").build(ui) {
            action = Action::Copy;
        }
        if MenuItem::new("Paste")
            .enabled(has_children && state.has_clipboard())
            .build(ui)
        {
            children
                .expect("paste with no children")
                .push(state.take_clipboard().expect("paste without clipboard"))
        }
        if MenuItem::new("Delete").build(ui) {
            action = Action::Delete;
        }

        action
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
        } else if let Some(children) = $self.children() {
            children
                .iter_mut()
                .any(|child| child.try_render_options($ui, $state))
        } else {
            false
        }
    };
}

pub(crate) use render_or_children;
