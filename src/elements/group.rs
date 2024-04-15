use super::{Element, RenderState};
use crate::{
    context::RenderContext,
    traits::{Node, Render, RenderOptions},
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Group {
    pub members: Vec<Element>,
}

impl Node for Group {
    fn children(&mut self) -> Option<&mut Vec<Element>> {
        Some(&mut self.members)
    }
}

impl Render for Group {
    fn render(&mut self, ui: &Ui, ctx: &RenderContext, state: &RenderState) {
        for member in &mut self.members {
            member.render(ui, ctx, state);
        }
    }
}

impl RenderOptions for Group {
    fn render_options(&mut self, _ui: &Ui) {}
}
