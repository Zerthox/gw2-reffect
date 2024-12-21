use crate::{
    context::Context,
    elements::{Common, Element, ElementType, Icon, IconElement, RenderState},
    render::{RenderDebug, RenderOptions},
    render_util::Rect,
    trigger::{FilterTrigger, ProgressTrigger, Trigger},
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

    pub filter: FilterTrigger,

    #[serde(flatten)]
    pub icon: Icon,
}

impl ListIcon {
    pub fn is_visible(&mut self, ctx: &Context, state: &RenderState) -> bool {
        let parent = state.common.trigger.active();
        self.trigger.update(ctx, state.is_edit(ctx), parent);
        self.enabled && self.trigger.active().is_some() && self.filter.is_active_or_edit(ctx, state)
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
            filter: self.filter,
            ..Element::default()
        }
    }

    pub fn from_element(common: Common, element: IconElement, filter: FilterTrigger) -> Self {
        Self {
            enabled: common.enabled,
            name: common.name,
            trigger: common.trigger,
            filter,
            icon: element.icon,
        }
    }
}

impl RenderOptions for ListIcon {
    fn render_options(&mut self, ui: &Ui, ctx: &Context) {
        ui.checkbox("Enabled", &mut self.enabled);
        ui.input_text("Name", &mut self.name).build();

        ui.spacing();

        self.trigger.render_options(ui, ctx);

        ui.spacing();

        self.icon.render_options(ui, ctx);
    }
}

impl RenderDebug for ListIcon {
    fn render_debug(&mut self, ui: &Ui, ctx: &Context) {
        self.icon.render_debug(ui, ctx)
    }
}

impl Default for ListIcon {
    fn default() -> Self {
        Self {
            enabled: true,
            name: "Unnamed".into(),
            trigger: ProgressTrigger::effect(),
            filter: FilterTrigger::default(),
            icon: Icon::default(),
        }
    }
}
