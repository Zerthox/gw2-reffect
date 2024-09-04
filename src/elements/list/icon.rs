use crate::{
    context::{Context, EditState},
    elements::{Common, Element, ElementType, Icon, IconElement, RenderState},
    render::{RenderDebug, RenderOptions},
    render_util::Rect,
    traits::{RenderDebug, RenderOptions},
    trigger::ProgressTrigger,
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ListIcon {
    pub enabled: bool,
    pub name: String,

    #[serde(alias = "buff")]
    #[serde(alias = "progress")]
    #[serde(alias = "progress_active")]
    pub trigger: ProgressTrigger,

    #[serde(flatten)]
    pub icon: Icon,
}

impl ListIcon {
    pub fn is_visible(&mut self, ctx: &Context, state: &RenderState) -> bool {
        let parent = state.common.trigger.active();
        self.trigger.update(ctx, state.is_edit(ctx), parent);
        self.enabled && self.trigger.active().is_some()
    }

    pub fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState, size: [f32; 2]) {
        self.icon
            .render(ui, ctx, state, self.trigger.active(), size)
    }

    pub fn bounds(&self, size: [f32; 2]) -> Rect {
        Icon::bounds(size)
    }

    pub fn into_element(self, size: [f32; 2]) -> Element {
        Element {
            common: Common {
                enabled: self.enabled,
                name: self.name,
                trigger: self.trigger,
                ..Common::default()
            },
            kind: ElementType::Icon(IconElement {
                icon: self.icon,
                size,
            }),
            ..Element::default()
        }
    }

    pub fn from_element(common: Common, element: IconElement) -> Self {
        Self {
            enabled: common.enabled,
            name: common.name,
            trigger: common.trigger,
            icon: element.icon,
        }
    }
}

impl RenderOptions for ListIcon {
    fn render_options(&mut self, ui: &Ui, state: &mut EditState) {
        ui.checkbox("Enabled", &mut self.enabled);
        ui.input_text("Name", &mut self.name).build();

        ui.spacing();

        self.trigger.render_options(ui, state);

        ui.spacing();

        self.icon.render_options(ui, state);
    }
}

impl RenderDebug for ListIcon {
    fn render_debug(&mut self, ui: &Ui) {
        self.icon.render_debug(ui)
    }
}

impl Default for ListIcon {
    fn default() -> Self {
        Self {
            enabled: true,
            name: "Unnamed".into(),
            trigger: ProgressTrigger::effect(),
            icon: Icon::default(),
        }
    }
}
