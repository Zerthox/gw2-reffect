use super::{Element, RenderCtx};
use crate::{
    context::Context,
    elements::Common,
    render::{Bounds, Rect},
    tree::TreeNode,
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Group {
    pub members: Vec<Element>,
}

impl TreeNode for Group {
    fn children(&mut self) -> Option<&mut Vec<Element>> {
        Some(&mut self.members)
    }
}

impl Group {
    pub fn render(&mut self, ui: &Ui, ctx: &RenderCtx, common: &Common) {
        for member in &mut self.members {
            member.render(ui, ctx, common);
        }
    }

    pub fn render_options(&mut self, _ui: &Ui, _ctx: &RenderCtx) {}

    pub fn render_tabs(&mut self, _ui: &Ui, _ctx: &RenderCtx) {}

    pub fn render_debug(&mut self, ui: &Ui, _ctx: &RenderCtx) {
        ui.text(format!("Members: {}", self.members.len()));
    }
}

impl Bounds for Group {
    fn bounds(&self, ui: &Ui, ctx: &Context) -> Rect {
        Bounds::combined_bounds(&self.members, ui, ctx)
    }
}
