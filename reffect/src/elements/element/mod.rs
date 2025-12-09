mod kind;

pub use self::kind::*;

use super::{Common, RenderCtx};
use crate::{
    action::ElementAction,
    colors,
    context::{Context, EditState},
    render::{
        Bounds, Rect, delete_confirm_modal, item_context_menu, style_disabled_if, tree_select_empty,
    },
    tree::{Loader, Resizer, TreeNode, VisitMut},
};
use nexus::imgui::{MenuItem, StyleColor, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Element {
    #[serde(flatten)]
    pub common: Common,

    #[serde(flatten)]
    pub kind: ElementType,
}

impl Element {
    pub fn of_type(kind: ElementType) -> Self {
        let mut element = Self {
            kind,
            ..Self::default()
        };
        Loader.visit_element(&mut element);
        element
    }

    /// Renders the element.
    pub fn render(&mut self, ui: &Ui, ctx: &RenderCtx, parent: &Common) {
        if self.common.is_visible(ctx) {
            if self.common.update(ctx, parent.trigger.active()) || self.kind.is_passthrough() {
                let _token = ctx.push_child(ui, &self.common);
                let _style = self.common.push_style(ui, ctx);
                self.kind.render(ui, ctx, &self.common);
            }

            if ctx.edit.is_edited(self.common.id) {
                let bounds = self.kind.bounds(ui, ctx);
                self.common.render_edit_indicators(ui, ctx.pos(), bounds);
            }
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
        let mut open_delete = false;
        let mut open_resize = false;

        item_context_menu(&id, || {
            self.common.render_context_menu(ui, children);

            if MenuItem::new("Cut").build(ui) {
                action = ElementAction::Cut;
            }
            if MenuItem::new("Copy").build(ui) {
                action = ElementAction::Copy;
            }
            if MenuItem::new("Duplicate").build(ui) {
                action = ElementAction::Duplicate;
            }
            if MenuItem::new("Move Up").build(ui) {
                action = ElementAction::Up;
            }
            if MenuItem::new("Move Down").build(ui) {
                action = ElementAction::Down;
            }
            open_resize = MenuItem::new("Resize").build(ui);

            let _color = ui.push_style_color(StyleColor::HeaderHovered, colors::DELETE_HOVER);
            open_delete = MenuItem::new("Delete").build(ui);
        });

        if let Some(factor) = self.common.render_resize(ui, open_resize) {
            Resizer::resize_element(self, factor);
        }

        let title = format!("Confirm Delete##reffect{id}");
        if open_delete {
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

        if token.is_some()
            && let Some(children) = self.kind.children()
        {
            selected |= self.common.render_tree_children(ui, state, children);
        }

        (selected, action)
    }

    /// Attempts to render options if selected.
    /// Returns `true` if the element or a child rendered.
    pub fn try_render_options(&mut self, ui: &Ui, ctx: &RenderCtx) -> bool {
        let id = self.common.id;
        if ctx.edit.is_selected(id) {
            self.render_options(ui, ctx);
            return true;
        } else if let (true, Some(children)) = (ctx.edit.is_selected_parent(id), self.children()) {
            for child in children {
                if child.try_render_options(ui, ctx) {
                    return true;
                }
            }
        }
        false
    }

    /// Renders the element options.
    pub fn render_options(&mut self, ui: &Ui, ctx: &RenderCtx) {
        if let Some(_token) = ui.tab_bar(self.common.id_string()) {
            if let Some(_token) = ui.tab_item(&self.kind) {
                self.common.render_options(ui, ctx);
                ui.spacing();
                self.kind.render_options(ui, ctx);
            }

            self.kind.render_tabs(ui, ctx, &self.common);

            if let Some(_token) = ui.tab_item("Filter") {
                self.common.render_filters(ui, ctx);
                self.kind.render_filters(ui, ctx);
            }

            if let Some(_token) = ui.tab_item("Animation") {
                self.common.render_animation(ui);
            }

            if let Some(_token) = ui.tab_item("?") {
                self.common.render_debug(ui, ctx);
                self.kind.render_debug(ui, ctx);
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
    fn bounds(&self, ui: &Ui, ctx: &Context) -> Rect {
        self.kind.bounds_with_offset(ui, ctx, self.common.pos)
    }
}
