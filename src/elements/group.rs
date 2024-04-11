use super::{Animation, Element, Node, Render};
use crate::{
    context::RenderContext,
    state::RenderState,
    trigger::{MetaTrigger, Trigger},
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Group {
    pub animation: Option<Animation>,
    pub condition: MetaTrigger,
    pub members: Vec<Element>,
}

impl Node for Group {
    fn children(&mut self) -> &mut [Element] {
        &mut self.members
    }
}

impl Render for Group {
    fn render(&mut self, ui: &Ui, ctx: &RenderContext, state: &mut RenderState) {
        if self.condition.is_active(ctx) {
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

    fn render_options(&mut self, ui: &Ui) {
        // TODO: group options
    }
}
