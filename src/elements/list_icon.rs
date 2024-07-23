use super::{Common, Element, ElementType, Icon, IconElement, RenderState};
use crate::{
    context::{Context, EditState},
    render_util::Rect,
    traits::RenderOptions,
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ListIcon {
    pub enabled: bool,
    pub name: String,

    #[serde(flatten)]
    pub inner: Icon,
}

impl ListIcon {
    pub fn is_visible(&mut self, ctx: &Context, state: &RenderState) -> bool {
        self.enabled && self.inner.is_visible(ctx, state)
    }

    pub fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState, size: [f32; 2]) {
        self.inner.render(ui, ctx, state, size)
    }

    pub fn bounds(&self, pos: [f32; 2], size: [f32; 2]) -> Rect {
        Icon::bounds(pos, size)
    }

    pub fn into_element(self, size: [f32; 2]) -> Element {
        Element {
            common: Common {
                enabled: self.enabled,
                name: self.name.clone(),
                ..Common::default()
            },
            kind: ElementType::Icon(IconElement {
                icon: self.inner.clone(),
                size,
            }),
            ..Element::default()
        }
    }

    pub fn from_element(common: Common, element: IconElement) -> Self {
        Self {
            enabled: common.enabled,
            name: common.name,
            inner: element.icon,
        }
    }
}

impl RenderOptions for ListIcon {
    fn render_options(&mut self, ui: &Ui, state: &mut EditState) {
        ui.checkbox("Enabled", &mut self.enabled);

        ui.input_text("Name", &mut self.name).build();

        self.inner.render_options(ui, state);
    }
}

impl Default for ListIcon {
    fn default() -> Self {
        Self {
            enabled: true,
            name: "Unnamed".into(),
            inner: Icon::default(),
        }
    }
}
