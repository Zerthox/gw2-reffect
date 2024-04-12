use super::{render_or_children, Common, ElementType, Node, Render, RenderState};
use crate::context::{EditState, RenderContext};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

// TODO: conditions, e.g. lower opacity out of combat, color change based on stack threshold

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Element {
    #[serde(flatten)]
    pub common: Common,

    #[serde(flatten)]
    pub kind: ElementType,
}

impl Element {
    /// Renders the element.
    pub fn render(&mut self, ui: &Ui, ctx: &RenderContext, state: &RenderState) {
        self.common
            .render(ui, ctx, state, |state| self.kind.render(ui, ctx, state))
    }

    /// Renders the select tree.
    /// Returns `true` if a child is selected.
    pub fn render_select_tree(&mut self, ui: &Ui, state: &mut EditState) -> bool {
        self.common
            .render_select_tree(ui, state, self.kind.type_name(), self.kind.children())
    }

    /// Attempts to render options if selected.
    /// Returns `true` if the element or a child rendered.
    pub fn try_render_options(&mut self, ui: &Ui, state: &EditState) -> bool {
        render_or_children!(self, ui, state)
    }

    /// Renders the element options.
    fn render_options(&mut self, ui: &Ui) {
        ui.group(|| {
            // TODO: tab bar?
            ui.text_disabled(format!("{} Options", self.kind.type_name()));
            ui.spacing();

            self.common.render_options(ui);

            self.kind.render_options(ui);
        });
    }
}

impl Node for Element {
    fn load(&mut self) {
        self.kind.load();
    }

    fn children(&mut self) -> &mut [Element] {
        self.kind.children()
    }
}
