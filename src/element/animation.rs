use super::{Element, Render, RenderState};
use crate::context::RenderContext;
use nexus::imgui::{StyleVar, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    /// Kind of animation.
    pub animation: AnimationKind,

    /// Animation period in milliseconds.
    pub period: u64,

    /// Elements being animated.
    pub elements: Vec<Element>,
}

impl Render for Animation {
    fn load(&mut self) {
        for element in &mut self.elements {
            element.load()
        }
    }

    fn render(&mut self, ui: &Ui, ctx: &RenderContext, state: &mut RenderState) {
        let time = (1000.0 * ui.time()) as u64;
        let passed = time % self.period;
        let progress = passed as f32 / self.period as f32;
        self.animation.animate(ui, progress, || {
            for element in &mut self.elements {
                element.render(ui, ctx, state)
            }
        });
    }
}

// TODO: tint animation changing color via state?
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum AnimationKind {
    Pulse,
}

impl AnimationKind {
    pub fn animate(&mut self, ui: &Ui, progress: f32, body: impl FnOnce()) {
        match self {
            Self::Pulse => {
                let prev = ui.clone_style().alpha;
                let factor = if progress < 0.5 {
                    1.0 - 2.0 * progress
                } else {
                    2.0 * progress - 1.0
                };
                let _token = ui.push_style_var(StyleVar::Alpha(factor * prev));
                body();
            }
        }
    }
}
