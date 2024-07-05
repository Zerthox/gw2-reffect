use super::{Icon, RenderState};
use crate::{context::Context, traits::RenderOptions};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

// TODO: individual opacity in list?

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ListIcon {
    pub name: String,

    #[serde(flatten)]
    pub inner: Icon,

    #[serde(skip)]
    pub open: bool,
}

impl ListIcon {
    pub fn is_visible(&mut self, ctx: &Context, state: &RenderState) -> bool {
        self.inner.is_visible(ctx, state)
    }

    pub fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState, size: [f32; 2]) {
        self.inner.render(ui, ctx, state, size)
    }
}

impl RenderOptions for ListIcon {
    fn render_options(&mut self, ui: &Ui) {
        ui.input_text("Name", &mut self.name).build();

        self.inner.render_options(ui);
    }
}

impl Default for ListIcon {
    fn default() -> Self {
        Self {
            name: "Unnamed".into(),
            inner: Icon::default(),
            open: false,
        }
    }
}
