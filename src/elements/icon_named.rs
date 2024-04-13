use super::{Icon, Node, RenderState};
use crate::context::RenderContext;
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IconNamed {
    pub name: String,

    #[serde(flatten)]
    pub inner: Icon,
}

impl IconNamed {
    pub fn render(&mut self, ui: &Ui, ctx: &RenderContext, state: &RenderState, size: [f32; 2]) {
        self.inner.render(ui, ctx, state, size)
    }

    pub fn render_options(&mut self, ui: &Ui) {
        ui.input_text("Name", &mut self.name).build();

        self.inner.render_options(ui);
    }
}

impl Node for IconNamed {
    fn load(&mut self) {
        self.inner.load();
    }

    fn children(&mut self) -> &mut [super::Element] {
        self.inner.children()
    }
}

impl Default for IconNamed {
    fn default() -> Self {
        Self {
            name: "Unnamed".into(),
            inner: Icon::default(),
        }
    }
}
