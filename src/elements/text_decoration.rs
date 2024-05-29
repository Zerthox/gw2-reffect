use crate::{
    component_wise::ComponentWise,
    render_util::{draw_text_bg, enum_combo},
};
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
    fn render_at(ui: &Ui, pos: [f32; 2], text: &str, font_size: f32, color: [f32; 4]) {
        draw_text_bg(ui, text, pos, font_size, color)
    }

    pub fn render(
        &self,
        ui: &Ui,
        text: impl AsRef<str>,
        pos: [f32; 2],
        font_size: f32,
        color: [f32; 4],
    ) {
        // FIXME: shadow renders behind transparent text
        let text = text.as_ref();
        match self {
            Self::None => {}
            Self::Shadow => Self::render_at(ui, pos.add([1.0, 1.0]), text, font_size, color),
            Self::ShadowDouble => {
                for offset in [[0.0, 1.0], [1.0, 0.0]] {
                    Self::render_at(ui, pos.add(offset), text, font_size, color)
                }
            }
            Self::Outline => {
                for offset in [[-1.0, -1.0], [1.0, 1.0]] {
                    Self::render_at(ui, pos.add(offset), text, font_size, color)
                }
            }
            Self::OutlineDouble => {
                for offset in [[-1.0, -1.0], [-1.0, 1.0], [1.0, -1.0], [1.0, 1.0]] {
                    Self::render_at(ui, pos.add(offset), text, font_size, color)
                }
            }
        }
    }

    pub fn render_select(&mut self, ui: &Ui) {
        enum_combo(ui, "Decoration", self, ComboBoxFlags::empty());
        if ui.is_item_hovered() {
            ui.tooltip_text("Warning: many text decorations may negatively impact performance");
        }
    }
}
