use crate::{
    action::DynAction,
    elements::{Common, Element, ElementType, Icon, IconElement, RenderCtx},
    render::Rect,
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
    pub fn is_visible(&mut self, ctx: &RenderCtx, common: &Common) -> bool {
        let parent = common.trigger.active();
        self.trigger.update(ctx, ctx.is_edited(), parent);
        self.enabled && self.trigger.active().is_some() && self.filter.is_active_or_edit(ctx)
    }

    pub fn render(&mut self, ui: &Ui, ctx: &RenderCtx, size: [f32; 2]) {
        self.icon.render(ui, ctx, self.trigger.active(), size)
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

impl ListIcon {
    pub fn render_options(&mut self, ui: &Ui, ctx: &RenderCtx) -> DynAction<Self> {
        ui.checkbox("Enabled", &mut self.enabled);
        ui.input_text("Name", &mut self.name).build();

        ui.spacing();

        self.trigger.render_options(ui);

        ui.spacing();

        let icon_action = self.icon.render_options(ui, ctx);
        icon_action.map(|list_icon: &mut Self| &mut list_icon.icon)
    }

    pub fn render_debug(&mut self, ui: &Ui, ctx: &RenderCtx) {
        self.trigger.render_debug(ui);
        self.filter.render_debug(ui, ctx);
        self.icon.render_debug(ui, ctx)
    }
}

impl Default for ListIcon {
    fn default() -> Self {
        Self {
            enabled: true,
            name: "Unnamed".into(),
            trigger: ProgressTrigger::buff(),
            filter: FilterTrigger::default(),
            icon: Icon::default(),
        }
    }
}
