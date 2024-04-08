use super::{Animation, Element, Render, RenderContext, RenderState};
use crate::trigger::{MetaTrigger, Trigger};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Group {
    pub name: String,
    pub offset: [f32; 2],
    pub animation: Option<Animation>,
    pub condition: MetaTrigger,
    pub members: Vec<Element>,
}

impl Render for Group {
    fn load(&mut self) {
        for member in &mut self.members {
            member.load();
        }
    }

    fn render(&mut self, ui: &Ui, ctx: &RenderContext, state: &mut RenderState) {
        if self.condition.is_active(ctx) {
            state.with_offset(self.offset, |state| {
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
            });
        }
    }
}

impl Default for Group {
    fn default() -> Self {
        Self {
            name: "Unnamed".into(),
            offset: [0.0, 0.0],
            animation: None,
            condition: MetaTrigger::default(),
            members: Vec::new(),
        }
    }
}
