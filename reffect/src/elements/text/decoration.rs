use crate::{
    math::ComponentWise,
    render::{draw_text_bg, enum_combo, helper_warn},
};
use const_default::ConstDefault;
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, VariantArray};

// TODO: avoid rendering same text multiple times, prerender characters as atlas?

/// Text decoration.
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
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum TextDecoration {
    /// No decoration-
    None,

    /// Simple shadow.
    Shadow,

    /// Thick shadow.
    #[strum(serialize = "Shadow (thick)")]
    ShadowDouble,

    /// Simple outline.
    Outline,

    /// Thick outline.
    #[strum(serialize = "Outline (thick)")]
    OutlineDouble,
}

impl ConstDefault for TextDecoration {
    const DEFAULT: Self = Self::None;
}

impl Default for TextDecoration {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl TextDecoration {
    /// Renders text with the given parameters.
    fn render_at(ui: &Ui, pos: [f32; 2], text: &str, font_scale: f32, color: [f32; 4]) {
        draw_text_bg(ui, text, pos, font_scale, color)
    }

    /// Renders the text decoration.
    pub fn render(
        &self,
        ui: &Ui,
        text: impl AsRef<str>,
        pos: [f32; 2],
        font_scale: f32,
        color: [f32; 4],
    ) {
        // FIXME: shadow renders behind transparent text
        let text = text.as_ref();
        match self {
            Self::None => {}
            Self::Shadow => Self::render_at(ui, pos.add([1.0, 1.0]), text, font_scale, color),
            Self::ShadowDouble => {
                for offset in [[0.0, 1.0], [1.0, 0.0]] {
                    Self::render_at(ui, pos.add(offset), text, font_scale, color)
                }
            }
            Self::Outline => {
                for offset in [[-1.0, -1.0], [1.0, 1.0]] {
                    Self::render_at(ui, pos.add(offset), text, font_scale, color)
                }
            }
            Self::OutlineDouble => {
                for offset in [[-1.0, -1.0], [-1.0, 1.0], [1.0, -1.0], [1.0, 1.0]] {
                    Self::render_at(ui, pos.add(offset), text, font_scale, color)
                }
            }
        }
    }

    /// Renders decoration selection.
    pub fn render_select(&mut self, ui: &Ui) {
        enum_combo(ui, "Decoration", self, ComboBoxFlags::empty());
        if *self != Self::None {
            helper_warn(ui, || {
                ui.text("Displaying a lot of text decorations may negatively impact performance")
            });
        }
    }
}
