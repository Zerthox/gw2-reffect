use super::{render_or_children, Animation, Common, ElementType, RenderState};
use crate::{
    action::Action,
    context::{EditState, RenderContext},
    traits::{Node, Render, RenderOptions},
    trigger::{MetaTrigger, Trigger},
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

// TODO: conditions, e.g. lower opacity out of combat, color change based on stack threshold

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Element {
    #[serde(flatten)]
    pub common: Common,

    pub trigger: MetaTrigger,

    pub animation: Option<Animation>,

    #[serde(flatten)]
    pub kind: ElementType,
}

impl Element {
    pub fn of_type(kind: ElementType) -> Self {
        Self {
            kind,
            ..Self::default()
        }
    }

    /// Renders the element.
    pub fn render(&mut self, ui: &Ui, ctx: &RenderContext, state: &RenderState) {
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
    pub fn render_select_tree(&mut self, ui: &Ui, state: &mut EditState) -> Action {
        let kind = (&self.kind).into(); // borrow here to keep ownership
        self.common
            .render_select_tree(ui, state, kind, self.kind.children())
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

            if let Some(_token) = ui.tab_item("Trigger") {
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

impl Node for Element {
    fn load(&mut self) {
        self.kind.load();
    }

    fn children(&mut self) -> Option<&mut Vec<Element>> {
        self.kind.children()
    }
}
