use super::{Animation, Element, RenderState};
use crate::{
    context::RenderContext,
    traits::{Node, Render, RenderOptions},
    trigger::{MetaTrigger, Trigger},
};
use nexus::imgui::{CollapsingHeader, Ui};
use serde::{Deserialize, Serialize};

// TODO: move animation & condition to element?

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Group {
    pub condition: MetaTrigger,
    pub animation: Option<Animation>,
    pub members: Vec<Element>,
}

impl Node for Group {
    fn children(&mut self) -> &mut [Element] {
        &mut self.members
    }
}

impl Render for Group {
    fn render(&mut self, ui: &Ui, ctx: &RenderContext, state: &RenderState) {
        if self.condition.is_active_or_edit(ctx, state) {
            let mut body = || {
                for member in &mut self.members {
                    member.render(ui, ctx, state);
                }
            };

            if let Some(animation) = &mut self.animation {
                animation.render(ui, body);
            } else {
                body();
            }
        }
    }
}

impl RenderOptions for Group {
    fn render_options(&mut self, ui: &Ui) {
        if CollapsingHeader::new("Condition").build(ui) {
            self.condition.render_options(ui);
        }

        if CollapsingHeader::new("Animation").build(ui) {
            if self.animation.is_some() {
                if ui.checkbox("Enabled", &mut true) {
                    self.animation = None;
                }
            } else if ui.checkbox("Enabled", &mut false) {
                self.animation = Some(Animation::default());
            }

            if let Some(animation) = &mut self.animation {
                animation.render_options(ui);
            }
        }
    }
}
