use super::{util::with_offset, Context, Element, Render};
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

    fn render(&mut self, ui: &Ui, ctx: &Context) {
        with_offset(ui, self.offset, || {
            for member in &mut self.members {
                member.render(ui, ctx);
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
