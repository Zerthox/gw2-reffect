use super::{Element, Icon, RenderState};
use crate::{
    context::RenderContext,
    traits::{Node, RenderOptions},
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IconNamed {
    pub name: String,

    #[serde(flatten)]
    pub inner: Icon,

    #[serde(skip)]
    pub open: bool,
}

impl IconNamed {
    pub fn is_visible(&mut self, ctx: &RenderContext, state: &RenderState) -> bool {
        self.inner.is_visible(ctx, state)
    }

    pub fn render(&mut self, ui: &Ui, ctx: &RenderContext, state: &RenderState, size: [f32; 2]) {
        self.inner.render(ui, ctx, state, size)
    }
}

impl Node for IconNamed {
    fn load(&mut self) {
        self.inner.load();
    }

    fn children(&mut self) -> Option<&mut Vec<Element>> {
        self.inner.children()
    }
}

impl RenderOptions for IconNamed {
    fn render_options(&mut self, ui: &Ui) {
        ui.input_text("Name", &mut self.name).build();

        self.inner.render_options(ui);
    }
}

impl Default for IconNamed {
    fn default() -> Self {
        Self {
            name: "Unnamed".into(),
            inner: Icon::default(),
            open: false,
        }
    }
}
