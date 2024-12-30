use crate::{
    context::Context,
    render::RenderOptions,
    render_util::{cycle_progress, enum_combo, helper, input_seconds, push_alpha_change},
    util::non_zero_u32,
};
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use std::num::NonZero;
use strum::{AsRefStr, EnumIter, VariantArray};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Animation {
    /// Kind of animation.
    pub kind: AnimationKind,

    /// Animation period in milliseconds.
    pub period: NonZero<u32>,
}

impl Animation {
    pub fn render(&mut self, ui: &Ui, body: impl FnOnce()) {
        let progress = cycle_progress(ui, self.period);
        self.kind.animate(ui, progress, body);
    }
}

impl RenderOptions for Animation {
    fn render_options(&mut self, ui: &Ui, _ctx: &Context) {
        enum_combo(ui, "Animation", &mut self.kind, ComboBoxFlags::empty());

        let mut period = self.period.get();
        if input_seconds(ui, "Period", &mut period) {
            self.period = NonZero::new(period).unwrap_or(non_zero_u32!(1));
        }
        helper(ui, || ui.text("Animation loop duration in seconds"));
    }
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            kind: AnimationKind::Pulse,
            period: non_zero_u32!(3000),
        }
    }
}

// TODO: tint animation changing color via state?
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    AsRefStr,
    EnumIter,
    VariantArray,
    Serialize,
    Deserialize,
)]
pub enum AnimationKind {
    Pulse,
}

impl AnimationKind {
    pub fn animate(&mut self, ui: &Ui, progress: f32, body: impl FnOnce()) {
        match self {
            // TODO: dont disappear completely
            Self::Pulse => {
                let factor = if progress < 0.5 {
                    1.0 - 2.0 * progress
                } else {
                    2.0 * progress - 1.0
                };
                let _style = push_alpha_change(ui, factor);
                body();
            }
        }
    }
}
