use super::{util::with_offset, Icon, Render, State};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IconElement {
    icon: Icon,
    size: [f32; 2],
    pos: [f32; 2],
}

impl Render for IconElement {
    fn load(&mut self) {
        self.icon.load();
    }

    fn render(&mut self, ui: &Ui, state: &State) {
        with_offset(ui, self.pos, || {
            if self.icon.needs_render(state) {
                self.icon.render(ui, self.size)
            }
        })
    }
}

impl Default for IconElement {
    fn default() -> Self {
        Self {
            icon: Icon::default(),
            size: [32.0, 32.0],
            pos: [0.0, 0.0],
        }
    }
}
