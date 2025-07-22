use super::{Icon, RenderCtx};
use crate::{
    context::Context,
    elements::Common,
    render::{Bounds, Rect, input_size},
    tree::TreeNode,
};
use const_default::ConstDefault;
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

impl IconElement {
    pub fn render(&mut self, ui: &Ui, ctx: &RenderCtx, common: &Common) {
        self.icon
            .render(ui, ctx, common.trigger.active(), self.size)
    }

    pub fn render_options(&mut self, ui: &Ui, ctx: &RenderCtx) {
        input_size(&mut self.size);

        self.icon.render_options(ui, ctx);
    }

    pub fn render_tabs(&mut self, ui: &Ui, ctx: &RenderCtx) {
        self.icon.render_tabs(ui, ctx);
    }

    pub fn render_debug(&mut self, ui: &Ui, ctx: &RenderCtx) {
        self.icon.render_debug(ui, ctx)
    }
}

impl Bounds for IconElement {
    fn bounds(&self, _ui: &Ui, _ctx: &Context) -> Rect {
        Icon::bounds(self.size)
    }
}

impl ConstDefault for IconElement {
    const DEFAULT: Self = Self {
        icon: Icon::DEFAULT,
        size: [32.0, 32.0],
    };
}

impl Default for IconElement {
    fn default() -> Self {
        Self::DEFAULT
    }
}
