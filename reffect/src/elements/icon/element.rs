use super::Icon;
use crate::{
    context::Context,
    elements::{Common, RenderCtx, align::Align},
    render::{Bounds, Rect, enum_combo, input_size},
    tree::TreeNode,
};
use const_default::ConstDefault;
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct IconElement {
    #[serde(flatten)]
    pub icon: Icon,

    /// Icon size.
    pub size: [f32; 2],

    /// Icon alignment.
    pub align: Align,
}

impl TreeNode for IconElement {}

impl IconElement {
    pub fn render(&mut self, ui: &Ui, ctx: &RenderCtx, common: &Common) {
        self.icon
            .render(ui, ctx, common.trigger.active(), self.size, self.align)
    }

    pub fn render_options(&mut self, ui: &Ui, ctx: &RenderCtx) {
        input_size(&mut self.size);

        enum_combo(ui, "Align", &mut self.align, ComboBoxFlags::empty());

        self.icon.render_options(ui, ctx);
    }

    pub fn render_tabs(&mut self, ui: &Ui, ctx: &RenderCtx, common: &Common) {
        self.icon.render_tabs(ui, ctx, &common.trigger);
    }

    pub fn render_debug(&mut self, ui: &Ui, ctx: &RenderCtx, common: &Common) {
        self.icon.render_debug(ui, ctx, &common.trigger)
    }
}

impl Bounds for IconElement {
    fn bounds(&self, _ui: &Ui, _ctx: &Context) -> Rect {
        self.align.bounds(self.size)
    }
}

impl ConstDefault for IconElement {
    const DEFAULT: Self = Self {
        icon: Icon::DEFAULT,
        size: [32.0, 32.0],
        align: Align::Center,
    };
}

impl Default for IconElement {
    fn default() -> Self {
        Self::DEFAULT
    }
}
