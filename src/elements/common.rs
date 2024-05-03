use super::{Element, ElementType, RenderState};
use crate::{
    action::ChildAction,
    component_wise::ComponentWise,
    context::{EditState, RenderContext},
    id::{Id, IdGen},
    render_util::{input_float_with_format, EnumStaticVariants},
    traits::RenderOptions,
};
use nexus::imgui::{InputTextFlags, MenuItem, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Common {
    #[serde(skip)]
    pub id: Id,

    pub name: String,
    pub pos: [f32; 2],
}

impl Common {
    pub fn id_string(&self) -> String {
        self.id.to_string()
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

        if ctx.edit.is_edited(self.id) {
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

    pub fn render_tree_label(&self, ui: &Ui, kind: &str) {
        ui.same_line();
        ui.text_disabled(kind);
        ui.same_line();
        ui.text(&self.name);
    }

    pub fn render_tree_children(
        &self,
        ui: &Ui,
        state: &mut EditState,
        children: &mut Vec<Element>,
    ) {
        let mut action = ChildAction::new();
        for (i, child) in children.iter_mut().enumerate() {
            action.or(i, child.render_select_tree(ui, state));
        }
        action.perform(state, children);
    }

    /// Renders common context menu options.
    pub fn render_context_menu(
        &mut self,
        ui: &Ui,
        state: &mut EditState,
        children: Option<&mut Vec<Element>>,
    ) {
        if let Some(children) = children {
            ui.menu("Create", || {
                for kind in ElementType::static_variants() {
                    let name = kind.as_ref();
                    if MenuItem::new(name).build(ui) {
                        let new = Element::of_type(kind.clone());
                        children.push(new);
                    }
                }
            });
            if MenuItem::new("Paste")
                .enabled(state.has_clipboard())
                .build(ui)
            {
                children.push(state.take_clipboard().expect("paste without clipboard"))
            }
        }
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
        input_float_with_format(
            "Position x",
            x,
            10.0,
            100.0,
            "%.2f",
            InputTextFlags::empty(),
        );
        input_float_with_format(
            "Position y",
            y,
            10.0,
            100.0,
            "%.2f",
            InputTextFlags::empty(),
        );
    }
}

impl Default for Common {
    fn default() -> Self {
        Self {
            id: IdGen::generate(),
            name: "Unnamed".into(),
            pos: [0.0, 0.0],
        }
    }
}

impl Clone for Common {
    fn clone(&self) -> Self {
        Self {
            id: IdGen::generate(), // we want a fresh id for the clone
            name: self.name.clone(),
            pos: self.pos,
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
