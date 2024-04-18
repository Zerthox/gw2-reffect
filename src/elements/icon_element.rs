use super::{Icon, RenderState};
use crate::{
    context::RenderContext,
    render_util::input_size,
    traits::{Leaf, Render, RenderOptions},
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IconElement {
    pub icon: Icon,
    pub size: [f32; 2],
}

impl Leaf for IconElement {
    fn load(&mut self) {
        self.icon.load();
    }

    fn context_update(&mut self, _ctx: &RenderContext) {}
}

impl Render for IconElement {
    fn render(&mut self, ui: &Ui, ctx: &RenderContext, state: &RenderState) {
        if self.icon.is_visible(ctx, state) {
            self.icon.render(ui, ctx, state, self.size)
        }
    }
}

impl RenderOptions for IconElement {
    fn render_options(&mut self, ui: &Ui) {
        let [x, y] = &mut self.size;
        input_size(x, y);

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
