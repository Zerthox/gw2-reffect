use super::{Animation, Common, ElementType, RenderState};
use crate::{
    action::ElementAction,
    bounds::Bounds,
    component_wise::ComponentWise,
    context::{Context, EditState},
    render_util::{
        delete_confirm_modal, item_context_menu, style_disabled_if, tree_select_empty, Rect,
    },
    traits::{Render, RenderOptions},
    tree::{Loader, TreeNode, VisitMut},
    trigger::{FilterTrigger, Trigger},
};
use nexus::imgui::{MenuItem, Ui};
use serde::{Deserialize, Serialize};

// TODO: conditions, e.g. lower opacity out of combat, color change based on stack threshold
// TODO: anchor to parent vs screen

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Element {
    #[serde(flatten)]
    pub common: Common,

    // TODO: move filter, animation to common to allow on pack? need to figure out pack render conditions, debug tab
    pub filter: FilterTrigger,

    pub animation: Option<Animation>,

    #[serde(flatten)]
    pub kind: ElementType,

    #[serde(skip)]
    pub confirm_delete: bool,
}

impl Element {
    pub fn of_type(kind: ElementType) -> Self {
        let mut element = Self {
            kind,
            ..Self::default()
        };
        element.load();
        element
    }

    pub fn load(&mut self) {
        Loader.visit_element(self);
    }

    /// Renders the element.
    pub fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState) {
        self.common.render_child(ui, ctx, state, |state| {
            if self.filter.is_active_or_edit(ctx, &state) {
                let mut body = || self.kind.render(ui, ctx, &state);
                if let Some(animation) = &mut self.animation {
                    animation.render(ui, body);
                } else {
                    body();
                }
            }
        });

        if ctx.edit.is_edited(self.common.id) {
            let pos = self.common.pos(state);
            let bounds = self.kind.bounding_box(ui, ctx, pos);
            self.common.render_edit_indicators(ui, pos, bounds);
        }
    }

    /// Renders the select tree.
    ///
    /// Returns `true` if the element was selected.
    pub fn render_select_tree(&mut self, ui: &Ui, state: &mut EditState) -> (bool, ElementAction) {
        let mut selected = false;
        let kind = (&self.kind).into(); // borrow here to keep ownership
        let id = self.common.id_string();
        let children = self.kind.children();
        let leaf = children
            .as_ref()
            .map(|children| children.is_empty())
            .unwrap_or(true);
        let (token, clicked) = {
            let _style = style_disabled_if(ui, !self.common.enabled);
            tree_select_empty(ui, &id, state.is_selected(self.common.id), leaf)
        };
        if clicked {
            selected = state.select(self.common.id);
        }

        let mut action = ElementAction::None;

        let mut open = false;
        item_context_menu(&id, || {
            self.common.render_context_menu(ui, state, children);

            if MenuItem::new("Cut").build(ui) {
                action = ElementAction::Cut;
            }
            if MenuItem::new("Copy").build(ui) {
                action = ElementAction::Copy;
            }
            if MenuItem::new("Move Up").build(ui) {
                action = ElementAction::Up;
            }
            if MenuItem::new("Move Down").build(ui) {
                action = ElementAction::Down;
            }
            open = MenuItem::new("Delete").build(ui);
        });
        let title = format!("Confirm Delete##reffect{id}");
        if open {
            ui.open_popup(&title)
        }
        if delete_confirm_modal(ui, &title, || {
            ui.text(format!("Delete {kind} {}?", self.common.name))
        }) {
            action = ElementAction::Delete;
        }

        {
            let _style = style_disabled_if(ui, !self.common.enabled);
            self.common.render_tree_label(ui, kind);
        }

        if token.is_some() {
            if let Some(children) = self.kind.children() {
                selected |= self.common.render_tree_children(ui, state, children);
            }
        }

        (selected, action)
    }

    /// Attempts to render options if selected.
    /// Returns `true` if the element or a child rendered.
    pub fn try_render_options(&mut self, ui: &Ui, state: &mut EditState) -> bool {
        let id = self.common.id;
        if state.is_selected(id) {
            self.render_options(ui, state);
            return true;
        } else if let (true, Some(children)) = (state.is_selected_parent(id), self.children()) {
            for child in children {
                if child.try_render_options(ui, state) {
                    return true;
                }
            }
        }
        false
    }

    /// Renders the element options.
    fn render_options(&mut self, ui: &Ui, state: &mut EditState) {
        if let Some(_token) = ui.tab_bar(self.common.id_string()) {
            if let Some(_token) = ui.tab_item(&self.kind) {
                self.common.render_options(ui, state);
                self.kind.render_options(ui, state);
            }

            if let Some(_token) = ui.tab_item("Filter") {
                self.filter.render_options(ui, state);
            }

            if let Some(_token) = ui.tab_item("Animation") {
                if self.animation.is_some() {
                    if ui.checkbox("Enabled", &mut true) {
                        self.animation = None;
                    }
                } else if ui.checkbox("Enabled", &mut false) {
                    self.animation = Some(Animation::default());
                }

                if let Some(animation) = &mut self.animation {
                    animation.render_options(ui, state);
                }
            }

            if let Some(_token) = ui.tab_item("?") {
                self.common.render_debug(ui);
            }
        }
    }
}

impl TreeNode for Element {
    fn children(&mut self) -> Option<&mut Vec<Element>> {
        self.kind.children()
    }
}

impl Bounds for Element {
    fn bounding_box(&self, ui: &Ui, ctx: &Context, pos: [f32; 2]) -> Rect {
        let pos = pos.add(self.common.pos);
        self.kind.bounding_box(ui, ctx, pos)
    }
}
