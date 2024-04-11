use super::{Element, Icon, Node, Render};
use crate::{context::RenderContext, state::RenderState};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IconElement {
    pub icon: Icon,
    pub size: [f32; 2],
}

impl Node for IconElement {
    fn load(&mut self) {
        self.icon.load();
    }

    fn children(&mut self) -> &mut [Element] {
        &mut []
    }
}

impl Render for IconElement {
    fn render(&mut self, ui: &Ui, ctx: &RenderContext, state: &mut RenderState) {
        if self.icon.is_active(ctx) {
            self.icon.render(ui, ctx, state.pos, self.size)
        }
    }

    fn render_options(&mut self, ui: &Ui) {
        ui.text("Icon");
        ui.same_line();
        self.icon.render_options(ui);
    }
}

impl Default for IconElement {
    fn default() -> Self {
        Self {
            icon: Icon::default(),
            size: [32.0, 32.0],
        }
    }
}
