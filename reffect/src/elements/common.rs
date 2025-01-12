use super::{Anchor, Element, ElementType, RenderState, ScreenAnchor, ELEMENT_ID};
use crate::{
    action::ChildElementAction,
    context::{Context, EditState},
    id::Id,
    render::{colors, ComponentWise, RenderDebug, RenderOptions},
    render_util::{
        helper_slider, input_pos, push_alpha_change, slider_percent, EnumStaticVariants, Rect,
    },
    serde::migrate,
    trigger::ProgressTrigger,
};
use nexus::imgui::{Condition, MenuItem, MouseButton, StyleVar, Ui, Window};
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

    #[serde(deserialize_with = "migrate::<_, _, ScreenAnchor>")]
    pub anchor: Anchor,
    pub pos: [f32; 2],

    pub opacity: f32,

    #[serde(alias = "buff")]
    #[serde(alias = "progress")]
    #[serde(alias = "progress_trigger")]
    pub trigger: ProgressTrigger,

    #[serde(skip)]
    pub dragging: bool,
}

impl Common {
    pub fn id_string(&self) -> String {
        self.id.to_string()
    }

    pub fn visible(&self, ctx: &Context) -> bool {
        self.enabled || ctx.edit.is_edited_or_parent(self.id)
    }

    pub fn pos(&self, ui: &Ui, parent_pos: [f32; 2]) -> [f32; 2] {
        self.anchor.pos(ui, parent_pos).add(self.pos)
    }

    pub fn render_root(
        &mut self,
        ui: &Ui,
        ctx: &Context,
        edit: bool,
        contents: impl FnOnce(RenderState),
    ) {
        if self.visible(ctx) {
            let edit = edit || ctx.edit.is_edited(self.id);
            self.trigger.update(ctx, edit, None);

            let _style = push_alpha_change(ui, self.opacity);
            let pos = self.pos(ui, Anchor::root(ui));
            let state = RenderState::initial(edit, pos, self);
            contents(state);
        }
    }

    pub fn render_child(
        &mut self,
        ui: &Ui,
        ctx: &Context,
        parent: &RenderState,
        contents: impl FnOnce(RenderState),
    ) {
        if self.visible(ctx) {
            let edit = parent.is_edit(ctx) || ctx.edit.is_edited(self.id);
            let parent_active = parent.common.trigger.active();
            self.trigger.update(ctx, edit, parent_active);

            let _style = push_alpha_change(ui, self.opacity);
            let state = parent.for_child(ui, ctx, self);
            contents(state);
        }
    }

    /// Renders the element edit indicators.
    ///
    /// Updates the position if moved.
    pub fn render_edit_indicators(&mut self, ui: &Ui, parent_pos: [f32; 2], bounds: Rect) {
        const ANCHOR_SIZE: f32 = 5.0;
        const ANCHOR_OFFSET: [f32; 2] = [0.5 * ANCHOR_SIZE, 0.5 * ANCHOR_SIZE];
        const COLOR: [f32; 4] = colors::WHITE;
        const COLOR_DRAG: [f32; 4] = colors::YELLOW;

        let anchor = self.pos(ui, parent_pos);
        let (min, max) = bounds;
        let min = anchor.add(min);
        let max = anchor.add(max);
        let window_pos = min;
        let window_size = max.sub(window_pos);
        let _style = ui.push_style_var(StyleVar::WindowPadding([0.0, 0.0]));
        Window::new("##reffect-edit")
            .position(
                window_pos,
                if self.dragging {
                    Condition::Never
                } else {
                    Condition::Always
                },
            )
            .size(window_size, Condition::Always)
            .resizable(false)
            .draw_background(false)
            .title_bar(false)
            .focus_on_appearing(false)
            .bring_to_front_on_focus(false)
            .nav_inputs(false)
            .nav_focus(false)
            .scrollable(false)
            .scroll_bar(false)
            .save_settings(false)
            .build(ui, || {
                let hover = ui.is_window_hovered();
                self.dragging = hover && ui.is_mouse_down(MouseButton::Left);
                let color = if self.dragging { COLOR_DRAG } else { COLOR };

                let draw_list = ui.get_foreground_draw_list();

                if hover {
                    draw_list.add_rect(min, max, color).build();
                }

                let start = anchor.sub(ANCHOR_OFFSET);
                let end = anchor.add(ANCHOR_OFFSET);
                draw_list.add_rect(start, end, color).filled(true).build();

                let text_pos = anchor.add([0.5 * ANCHOR_SIZE + 1.0, 0.0]);
                draw_list.add_text(text_pos, color, &self.name);

                if self.dragging {
                    let change = ui.window_pos().sub(window_pos);
                    let [new_x, new_y] = self.pos.add(change);
                    self.pos = [new_x.round(), new_y.round()];
                }
            });
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
        action.perform(children, state);
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
                ElementType::with_variants(|variants| {
                    for kind in variants {
                        let name = kind.as_ref();
                        if MenuItem::new(name).build(ui) {
                            let new = Element::of_type(kind.clone());
                            children.push(new);
                        }
                    }
                })
            });
            if MenuItem::new("Paste")
                .enabled(state.clipboard.has_some())
                .build(ui)
            {
                children.push(state.clipboard.take().expect("paste without clipboard"))
            }
        }
    }
}

impl RenderOptions for Common {
    fn render_options(&mut self, ui: &Ui, ctx: &Context) {
        ui.checkbox("Enabled", &mut self.enabled);

        ui.input_text("Name", &mut self.name).build();

        self.anchor.render_select(ui);
        input_pos(&mut self.pos);

        slider_percent(ui, "Opacity", &mut self.opacity);
        helper_slider(ui);

        ui.spacing();

        self.trigger.render_options(ui, ctx);
    }
}

impl RenderDebug for Common {
    fn render_debug(&mut self, ui: &Ui, ctx: &Context) {
        ui.text(format!("Id: {}", self.id));
        ui.text(format!("Pos: {:?}", self.pos(ui, Anchor::root(ui))));

        self.trigger.render_debug(ui, ctx);
    }
}

impl Default for Common {
    fn default() -> Self {
        Self {
            enabled: true,
            id: ELEMENT_ID.generate(),
            name: "Unnamed".into(),
            anchor: Anchor::default(),
            pos: [0.0, 0.0],
            opacity: 1.0,
            trigger: ProgressTrigger::default(),
            dragging: false,
        }
    }
}

impl Clone for Common {
    fn clone(&self) -> Self {
        Self {
            enabled: self.enabled,
            id: ELEMENT_ID.generate(), // we want a fresh id for the clone
            name: self.name.clone(),
            anchor: self.anchor,
            pos: self.pos,
            opacity: self.opacity,
            trigger: self.trigger.clone(),
            dragging: false,
        }
    }
}
