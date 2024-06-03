use super::{Element, RenderState};
use crate::{
    context::Context,
    traits::{Render, RenderOptions, TreeNode},
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

// TODO: opacity

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

impl RenderOptions for Group {
    fn render_options(&mut self, _ui: &Ui) {}
}
