use super::{Element, RenderCtx};
use crate::{
    context::Context,
    elements::Common,
    render::{Bounds, Rect},
    tree::TreeNode,
};
use const_default::ConstDefault;
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

/// Group element.
#[derive(Debug, Default, ConstDefault, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct Group {
    /// Group member elements.
    pub members: Vec<Element>,
}

impl TreeNode for Group {
    fn children(&mut self) -> Option<&mut Vec<Element>> {
        Some(&mut self.members)
    }
}

impl Group {
    /// Renders the group.
    pub fn render(&mut self, ui: &Ui, ctx: &RenderCtx, _common: &Common) {
        for member in &mut self.members {
            member.render(ui, ctx);
        }
    }

    /// Renders group options.
    pub fn render_options(&mut self, _ui: &Ui, _ctx: &RenderCtx) {}

    /// Renders group tabs.
    pub fn render_tabs(&mut self, _ui: &Ui, _ctx: &RenderCtx) {}

    /// Renders group debug information.
    pub fn render_debug(&mut self, ui: &Ui, _ctx: &RenderCtx) {
        ui.text(format!("Members: {}", self.members.len()));
    }
}

impl Bounds for Group {
    fn bounds(&self, ui: &Ui, ctx: &Context) -> Rect {
        Bounds::combined_bounds(&self.members, ui, ctx)
    }
}
