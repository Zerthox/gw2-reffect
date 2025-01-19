use super::{Icon, RenderState};
use crate::{
    context::Context,
    render::{input_size, Bounds, Rect, Render, RenderDebug, RenderOptions},
    tree::TreeNode,
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IconElement {
    #[serde(flatten)]
    pub icon: Icon,
    pub size: [f32; 2],
}

impl TreeNode for IconElement {}

impl Render for IconElement {
    fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState) {
        self.icon
            .render(ui, ctx, state, state.trigger_active(), self.size)
    }
}

impl Bounds for IconElement {
    fn bounds(&self, _ui: &Ui, _ctx: &Context) -> Rect {
        Icon::bounds(self.size)
    }
}

impl RenderOptions for IconElement {
    fn render_options(&mut self, ui: &Ui, ctx: &Context) {
        input_size(&mut self.size);

        self.icon.render_options(ui, ctx);
    }

    fn render_tabs(&mut self, ui: &Ui, ctx: &Context) {
        self.icon.render_tabs(ui, ctx);
    }
}

impl RenderDebug for IconElement {
    fn render_debug(&mut self, ui: &Ui, ctx: &Context) {
        self.icon.render_debug(ui, ctx)
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
