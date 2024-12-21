use super::{Element, RenderState};
use crate::{
    context::Context,
    render::{Bounds, Render, RenderDebug, RenderOptions},
    render_util::Rect,
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

impl Render for Group {
    fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState) {
        for member in &mut self.members {
            member.render(ui, ctx, state);
        }
    }
}

impl Bounds for Group {
    fn bounds(&self, ui: &Ui, ctx: &Context) -> Rect {
        Bounds::combined_bounds(&self.members, ui, ctx)
    }
}

impl RenderOptions for Group {
    fn render_options(&mut self, _ui: &Ui, _ctx: &Context) {}
}

impl RenderDebug for Group {
    fn render_debug(&mut self, ui: &Ui, _ctx: &Context) {
        ui.text(format!("Members: {}", self.members.len()));
    }
}
