use super::{Anchor, ELEMENT_ID, Element, ElementType, ScreenAnchor};
use crate::{
    action::ChildElementAction,
    clipboard::Clipboard,
    colors,
    context::{Context, EditState},
    elements::RenderCtx,
    enums::EnumStaticVariants,
    id::Id,
    render::{
        ComponentWise, Rect, confirm_modal, helper_slider, input_percent, input_pos,
        push_alpha_change, push_window_clip_rect_fullscreen, slider_percent,
    },
    serde::migrate,
    trigger::{FilterTrigger, ProgressActive, ProgressTrigger, Trigger},
};
use nexus::imgui::{Condition, MenuItem, MouseButton, StyleVar, Ui, Window};
use serde::{Deserialize, Serialize};
use std::mem;

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

    pub filter: FilterTrigger,

    #[serde(skip)]
    pub dragging: bool,

    #[serde(skip)]
    pub resize: f32,
}

impl Common {
    pub fn id_string(&self) -> String {
        self.id.to_string()
    }

    pub fn is_visible(&mut self, ctx: &RenderCtx) -> bool {
        if ctx.edit.is_editing() {
            (self.enabled && ctx.is_edited()) || ctx.edit.is_selected_or_parent(self.id)
        } else {
            self.enabled && self.filter.is_active(ctx)
        }
    }

    pub fn pos(&self, ui: &Ui, parent_pos: [f32; 2]) -> [f32; 2] {
        self.anchor.pos(ui, parent_pos).add(self.pos)
    }

    pub fn pos_root(&self, ui: &Ui) -> [f32; 2] {
        self.pos(ui, Anchor::root(ui))
    }

    pub fn update(&mut self, ctx: &Context, parent_active: Option<&ProgressActive>) -> bool {
        self.trigger.update(ctx, parent_active)
    }

    pub fn push_style<'ui>(&self, ui: &'ui Ui) -> impl Drop + 'ui {
        push_alpha_change(ui, self.opacity)
    }

    /// Renders the element edit indicators.
    ///
    /// Updates the position if moved.
    pub fn render_edit_indicators(&mut self, ui: &Ui, parent_pos: [f32; 2], bounds: Rect) {
        const ANCHOR_HALF_SIZE: f32 = 2.0;
        const COLOR: [f32; 4] = colors::WHITE;
        const COLOR_DRAG: [f32; 4] = colors::YELLOW;
        const COLOR_SHADOW: [f32; 4] = colors::with_alpha(colors::BLACK, 0.7);

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

                let draw_list = ui.get_window_draw_list();
                let _clip = push_window_clip_rect_fullscreen(ui);

                if hover {
                    draw_list
                        .add_rect(min.add_scalar(1.0), max.add_scalar(1.0), COLOR_SHADOW)
                        .build();
                    draw_list.add_rect(min, max, color).build();
                }

                let start = anchor.sub_scalar(ANCHOR_HALF_SIZE);
                let end = anchor.add_scalar(ANCHOR_HALF_SIZE);
                draw_list
                    .add_rect(start.sub_scalar(1.0), end.add_scalar(1.0), COLOR_SHADOW)
                    .filled(true)
                    .build();
                draw_list.add_rect(start, end, color).filled(true).build();

                let text_pos = anchor.add([ANCHOR_HALF_SIZE + 1.0, 0.0]);
                draw_list.add_text(text_pos.add_scalar(1.0), COLOR_SHADOW, &self.name);
                draw_list.add_text(text_pos, color, &self.name);

                if self.dragging {
                    let change = ui.window_pos().sub(window_pos);
                    let [new_x, new_y] = self.pos.add(change);
                    self.pos = [new_x.round_ties_even(), new_y.round_ties_even()];
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
        action.perform(children);
        selected
    }

    /// Renders common context menu options.
    pub fn render_context_menu(&mut self, ui: &Ui, children: Option<&mut Vec<Element>>) {
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
                .enabled(Clipboard::has_some())
                .build(ui)
            {
                children.push(Clipboard::take().expect("paste without clipboard"))
            }
        }
    }

    pub fn render_resize(&mut self, ui: &Ui, open: bool) -> Option<f32> {
        let title = format!("Resize Element##reffect{}", self.id);
        if open {
            ui.open_popup(&title)
        }
        confirm_modal(ui, &title, || {
            input_percent("Scale", &mut self.resize);
        })
        .then(|| mem::replace(&mut self.resize, 1.0))
    }

    pub fn render_options(&mut self, ui: &Ui) {
        ui.checkbox("Enabled", &mut self.enabled);

        ui.input_text("Name", &mut self.name).build();

        self.anchor.render_select(ui);
        input_pos(&mut self.pos);

        slider_percent(ui, "Opacity", &mut self.opacity);
        helper_slider(ui);

        ui.spacing();

        self.trigger.render_options(ui);
    }

    pub fn render_filters(&mut self, ui: &Ui, ctx: &RenderCtx) {
        self.filter.render_options(ui, ctx);
    }

    pub fn render_debug(&mut self, ui: &Ui, ctx: &RenderCtx) {
        ui.text(format!("Id: {}", self.id));
        ui.text(format!("Pos: {:?}", self.pos_root(ui)));

        self.trigger.render_debug(ui);
        self.filter.render_debug(ui, ctx);
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
            filter: FilterTrigger::default(),
            dragging: false,
            resize: 1.0,
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
            filter: self.filter.clone(),
            dragging: false,
            resize: 1.0,
        }
    }
}
