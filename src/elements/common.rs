use super::{Element, ElementType, RenderState};
use crate::{
    action::ChildElementAction,
    component_wise::ComponentWise,
    context::{Context, EditState},
    id::{Id, IdGen},
    render_util::{helper, input_float_with_format, push_alpha_change, EnumStaticVariants},
    traits::RenderOptions,
};
use nexus::imgui::{InputTextFlags, MenuItem, Slider, SliderFlags, Ui};
use serde::{Deserialize, Serialize};

// FIXME: common default is called twice when deserializing element/pack, generating unused ids

/// Common behavior between elements and packs.
#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Common {
    pub enabled: bool,

    #[serde(skip)]
    pub id: Id,

    pub name: String,

    pub pos: [f32; 2],

    pub opacity: f32,
}

impl Common {
    pub fn id_string(&self) -> String {
        self.id.to_string()
    }

    pub fn render(
        &self,
        ui: &Ui,
        ctx: &Context,
        state: &RenderState,
        contents: impl FnOnce(&RenderState),
    ) {
        let show = if ctx.edit.is_editing() {
            ctx.edit.is_edited_or_parent(self.id)
        } else {
            self.enabled
        };
        if show {
            let state = state.with_offset(self.pos).for_element(self, ctx);
            {
                let _style = push_alpha_change(ui, self.opacity);
                contents(&state);
            }

            if ctx.edit.is_edited(self.id) {
                // TODO: change edited element draw list to foreground?
                self.render_edit_indicator(ui, &state);
            }
        }
    }

    fn render_edit_indicator(&self, ui: &Ui, state: &RenderState) {
        const SIZE: f32 = 5.0;
        const HALF_SIZE: f32 = 0.5 * SIZE;
        const OFFSET: [f32; 2] = [HALF_SIZE, HALF_SIZE];
        const COLOR: [f32; 4] = [1.0, 0.0, 0.0, 0.8];

        let draw_list = ui.get_foreground_draw_list();

        let start = state.pos.sub(OFFSET);
        let end = state.pos.add(OFFSET);
        draw_list.add_rect(start, end, COLOR).filled(true).build();

        let pos = state.pos.add([HALF_SIZE + 1.0, 0.0]);
        draw_list.add_text(pos, COLOR, &self.name);
    }

    pub fn render_tree_label(&self, ui: &Ui, kind: &str) {
        ui.same_line();
        ui.text_disabled(kind);
        ui.same_line();
        ui.text(&self.name);
    }

    /// Renders the select tree children.
    ///
    /// Returns `true` if a child was selected.
    pub fn render_tree_children(
        &self,
        ui: &Ui,
        state: &mut EditState,
        children: &mut Vec<Element>,
    ) -> bool {
        let mut selected = false;
        let mut action = ChildElementAction::new();
        for (i, child) in children.iter_mut().enumerate() {
            let (child_selected, child_action) = child.render_select_tree(ui, state);
            action.or(i, child_action);
            if child_selected {
                state.push_parent(self.id);
                selected = true;
            }
        }
        action.perform(state, children);
        selected
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
        ui.checkbox("Enabled", &mut self.enabled);

        ui.input_text("Name", &mut self.name).build();

        let [x, y] = &mut self.pos;
        input_float_with_format("Position x", x, 1.0, 10.0, "%.2f", InputTextFlags::empty());
        input_float_with_format("Position y", y, 1.0, 10.0, "%.2f", InputTextFlags::empty());

        let mut opacity = 100.0 * self.opacity;
        if Slider::new("Opacity", 0.0, 100.0)
            .flags(SliderFlags::ALWAYS_CLAMP)
            .display_format("%.2f")
            .build(ui, &mut opacity)
        {
            self.opacity = opacity / 100.0;
        }
        ui.same_line();
        helper(ui, || ui.text("Ctrl+Click to type a number"));

        ui.spacing();
    }
}

impl Default for Common {
    fn default() -> Self {
        Self {
            enabled: true,
            id: IdGen::generate(),
            name: "Unnamed".into(),
            pos: [0.0, 0.0],
            opacity: 1.0,
        }
    }
}

impl Clone for Common {
    fn clone(&self) -> Self {
        Self {
            enabled: self.enabled,
            id: IdGen::generate(), // we want a fresh id for the clone
            name: self.name.clone(),
            pos: self.pos,
            opacity: self.opacity,
        }
    }
}
