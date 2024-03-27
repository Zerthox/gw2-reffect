use super::{util::with_offset, Element, Render, State};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Group {
    pub name: String,
    pub pos: [f32; 2],
    pub members: Vec<Element>,
}

impl Render for Group {
    fn load(&mut self) {
        for member in &mut self.members {
            member.load();
        }
    }

    fn render(&mut self, ui: &Ui, state: &State) {
        with_offset(ui, self.pos, || {
            for member in &mut self.members {
                member.render(ui, state);
            }
        })
    }
}

impl Default for Group {
    fn default() -> Self {
        Self {
            name: "Unnamed".into(),
            members: Vec::new(),
            pos: [0.0, 0.0],
        }
    }
}
