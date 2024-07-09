use super::{Element, RenderState};
use crate::{
    bounds::Bounds,
    context::Context,
    render_util::Rect,
    traits::{Render, RenderOptions},
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
    fn bounding_box(&self, ui: &Ui, ctx: &Context, pos: [f32; 2]) -> Rect {
        Bounds::combined_bounds(&self.members, ui, ctx, pos)
    }
}

impl RenderOptions for Group {
    fn render_options(&mut self, _ui: &Ui) {}
}
