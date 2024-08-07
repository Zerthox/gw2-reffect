use super::{Icon, RenderState};
use crate::{
    bounds::Bounds,
    context::{Context, EditState},
    render_util::{input_size, Rect},
    traits::{Render, RenderDebug, RenderOptions},
    tree::TreeLeaf,
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

impl TreeLeaf for IconElement {}

impl Render for IconElement {
    fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState) {
        self.icon
            .render(ui, ctx, state, state.trigger_active(), self.size)
    }
}

impl Bounds for IconElement {
    fn bounding_box(&self, _ui: &Ui, _ctx: &Context, pos: [f32; 2]) -> Rect {
        Icon::bounds(pos, self.size)
    }
}

impl RenderOptions for IconElement {
    fn render_options(&mut self, ui: &Ui, state: &mut EditState) {
        input_size(&mut self.size);

        self.icon.render_options(ui, state);
    }

    fn render_tabs(&mut self, ui: &Ui, state: &mut EditState) {
        self.icon.render_tabs(ui, state);
    }
}

impl RenderDebug for IconElement {
    fn render_debug(&mut self, ui: &Ui) {
        self.icon.render_debug(ui)
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
