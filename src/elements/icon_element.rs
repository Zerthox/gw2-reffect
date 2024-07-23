use super::{Icon, RenderState};
use crate::{
    bounds::Bounds,
    context::{Context, EditState},
    render_util::{input_size, Rect},
    traits::{Render, RenderOptions},
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
        if self.icon.is_visible(ctx, state) {
            self.icon.render(ui, ctx, state, self.size)
        }
    }
}

impl Bounds for IconElement {
    fn bounding_box(&self, _ui: &Ui, _ctx: &Context, pos: [f32; 2]) -> Rect {
        Icon::bounds(pos, self.size)
    }
}

impl RenderOptions for IconElement {
    fn render_options(&mut self, ui: &Ui, state: &mut EditState) {
        let [x, y] = &mut self.size;
        input_size(x, y);

        self.icon.render_options(ui, state);
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
