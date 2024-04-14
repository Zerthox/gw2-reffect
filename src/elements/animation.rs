use crate::{
    render_util::{enum_combo, input_u32},
    traits::RenderOptions,
};
use nexus::imgui::{ComboBoxFlags, StyleVar, Ui};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, VariantArray};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Animation {
    /// Kind of animation.
    pub kind: AnimationKind,

    /// Animation period in milliseconds.
    pub period: u32,
}

impl Animation {
    pub fn render(&mut self, ui: &Ui, body: impl FnOnce()) {
        let time = (1000.0 * ui.time()) as u32;
        let passed = time % self.period;
        let progress = passed as f32 / self.period as f32;
        self.kind.animate(ui, progress, body);
    }
}

impl RenderOptions for Animation {
    fn render_options(&mut self, ui: &Ui) {
        enum_combo(ui, "Animation", &mut self.kind, unsafe {
            ComboBoxFlags::from_bits_unchecked(1 >> 7)
        });

        input_u32(ui, "Period", &mut self.period);
    }
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            kind: AnimationKind::Pulse,
            period: 3000,
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
