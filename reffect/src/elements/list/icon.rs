use crate::{
    action::DynAction,
    elements::{Common, Element, ElementType, Icon, IconElement, RenderCtx, align::Align},
    trigger::{FilterTrigger, ProgressTrigger},
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct ListIcon {
    /// Whether the list icon is enabled.
    pub enabled: bool,

    /// Custom name for editor.
    pub name: String,

    /// Trigger configuration.
    #[serde(alias = "buff")]
    #[serde(alias = "progress")]
    #[serde(alias = "progress_active")]
    pub trigger: ProgressTrigger,

    /// Filter configuration.
    pub filter: FilterTrigger,

    #[serde(flatten)]
    pub icon: Icon,
}

impl ListIcon {
    pub const ALIGN: Align = Align::Center;

    pub fn is_visible(&mut self, ctx: &RenderCtx) -> bool {
        self.enabled
            && if ctx.edit.is_editing() {
                ctx.is_edit_visible()
            } else {
                self.filter.is_active(ctx) && self.trigger.is_visible()
            }
    }

    pub fn render(&mut self, ui: &Ui, ctx: &RenderCtx, size: [f32; 2]) {
        self.icon
            .render(ui, ctx, self.trigger.active(), size, Self::ALIGN)
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
                ..IconElement::default()
            }),
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
        self.icon.render_debug(ui, ctx, &self.trigger)
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
