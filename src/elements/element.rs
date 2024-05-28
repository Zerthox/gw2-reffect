use super::{render_or_children, Animation, Common, ElementType, RenderState};
use crate::{
    action::ElementAction,
    context::{Context, EditState},
    render_util::{delete_confirm_modal, item_context_menu, tree_select_empty},
    traits::{Render, RenderOptions, TreeNode},
    trigger::{MetaTrigger, Trigger},
    visit::{Loader, VisitMut},
};
use nexus::imgui::{MenuItem, Ui};
use serde::{Deserialize, Serialize};

// TODO: conditions, e.g. lower opacity out of combat, color change based on stack threshold

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Element {
    #[serde(flatten)]
    pub common: Common,

    // TODO: move to common? allow on pack?
    pub trigger: MetaTrigger,

    // TODO: move to common? allow on pack?
    pub animation: Option<Animation>,

    #[serde(flatten)]
    pub kind: ElementType,

    #[serde(skip)]
    confirm_delete: bool,
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
        if self.trigger.is_active_or_edit(ctx, state) {
            let mut body = || {
                self.common
                    .render(ui, ctx, state, |state| self.kind.render(ui, ctx, state))
            };

            if let Some(animation) = &mut self.animation {
                animation.render(ui, body);
            } else {
                body();
            }
        }
    }

    /// Renders the select tree.
    /// Returns `true` if a child is selected.
    pub fn render_select_tree(&mut self, ui: &Ui, state: &mut EditState) -> ElementAction {
        let kind = (&self.kind).into(); // borrow here to keep ownership
        let id = self.common.id_string();
        let selected = state.is_selected(self.common.id);
        let children = self.kind.children();
        let leaf = children
            .as_ref()
            .map(|children| children.is_empty())
            .unwrap_or(true);
        let (token, clicked) = tree_select_empty(ui, &id, selected, leaf);
        if clicked {
            state.select(self.common.id);
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
        let title = format!("Confirm Delete##{id}");
        if open {
            ui.open_popup(&title)
        }
        if delete_confirm_modal(ui, &title, || {
            ui.text(format!("Delete {kind} {}?", self.common.name))
        }) {
            action = ElementAction::Delete;
        }

        self.common.render_tree_label(ui, kind);
        if token.is_some() {
            if let Some(children) = self.kind.children() {
                self.common.render_tree_children(ui, state, children);
            }
        }

        action
    }

    /// Attempts to render options if selected.
    /// Returns `true` if the element or a child rendered.
    pub fn try_render_options(&mut self, ui: &Ui, state: &EditState) -> bool {
        render_or_children!(self, ui, state)
    }

    /// Renders the element options.
    fn render_options(&mut self, ui: &Ui) {
        if let Some(_token) = ui.tab_bar(self.common.id_string()) {
            if let Some(_token) = ui.tab_item(&self.kind) {
                self.common.render_options(ui);
                self.kind.render_options(ui);
            }

            if let Some(_token) = ui.tab_item("Filter") {
                self.trigger.render_options(ui);
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
                    animation.render_options(ui);
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
