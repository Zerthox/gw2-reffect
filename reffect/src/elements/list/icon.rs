use crate::{
    action::DynAction,
    context::Context,
    elements::{Common, Element, ElementType, Icon, IconElement, RenderCtx},
    render::Rect,
    trigger::{FilterTrigger, ProgressActive, ProgressTrigger, Trigger},
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
    pub fn is_visible(&mut self, ctx: &RenderCtx) -> bool {
        self.enabled
            && if ctx.edit.is_editing() {
                ctx.is_edited()
            } else {
                self.filter.is_active(ctx)
            }
    }

    pub fn update(&mut self, ctx: &Context, parent_active: Option<&ProgressActive>) -> bool {
        self.trigger.update(ctx, parent_active)
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
                filter: self.filter,
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
            filter: common.filter,
            icon: element.icon,
        }
    }
}

impl ListIcon {
    pub fn render_options(&mut self, ui: &Ui, ctx: &RenderCtx) -> DynAction<Self> {
        ui.checkbox("Enabled", &mut self.enabled);
        ui.input_text("Name", &mut self.name).build();

        ui.spacing();

        self.trigger.render_options(ui, ctx);

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
