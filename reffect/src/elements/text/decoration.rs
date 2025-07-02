use crate::render::{ComponentWise, draw_text_bg, enum_combo, helper_warn};
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, VariantArray};

// TODO: avoid rendering same text multiple times, prerender characters as atlas?

#[derive(
    Debug,
    Default,
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
pub enum TextDecoration {
    #[default]
    None,

    Shadow,

    #[strum(serialize = "Shadow (thick)")]
    ShadowDouble,

    Outline,

    #[strum(serialize = "Outline (thick)")]
    OutlineDouble,
}

impl TextDecoration {
    fn render_at(ui: &Ui, pos: [f32; 2], text: &str, font_scale: f32, color: [f32; 4]) {
        draw_text_bg(ui, text, pos, font_scale, color)
    }

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

    pub fn render_select(&mut self, ui: &Ui) {
        enum_combo(ui, "Decoration", self, ComboBoxFlags::empty());
        if *self != Self::None {
            helper_warn(ui, || {
                ui.text("Displaying a lot of text decorations may negatively impact performance")
            });
        }
    }
}
