use super::{Context, Element, Render, RenderState};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Group {
    pub name: String,
    pub offset: [f32; 2],
    pub members: Vec<Element>,
}

impl Render for Group {
    fn load(&mut self) {
        for member in &mut self.members {
            member.load();
        }
    }

    fn render(&mut self, ui: &Ui, ctx: &Context, state: &mut RenderState) {
        state.with_offset(self.offset, |state| {
            for member in &mut self.members {
                member.render(ui, ctx, state);
            }
        })
    }
}

impl Default for Group {
    fn default() -> Self {
        Self {
            name: "Unnamed".into(),
            members: Vec::new(),
            offset: [0.0, 0.0],
        }
    }
}
