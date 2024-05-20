use crate::{component_wise::ComponentWise, render_util::enum_combo};
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
    Outline,
}

impl TextDecoration {
    fn render_at(ui: &Ui, pos: [f32; 2], text: &str, color: [f32; 4]) {
        ui.set_cursor_pos(pos);
        ui.text_colored(color, text);
    }

    pub fn render(&self, ui: &Ui, text: impl AsRef<str>, color: [f32; 4]) {
        // FIXME: shadow renders behind transparent text
        let text = text.as_ref();
        let cursor = ui.cursor_pos();
        match self {
            TextDecoration::None => {}
            TextDecoration::Shadow => {
                for offset in [[0.0, 1.0], [1.0, 0.0]] {
                    Self::render_at(ui, cursor.add(offset), text, color)
                }
            }
            TextDecoration::Outline => {
                for offset in [[-1.0, -1.0], [-1.0, 1.0], [1.0, -1.0], [1.0, 1.0]] {
                    Self::render_at(ui, cursor.add(offset), text, color)
                }
            }
        }
        ui.set_cursor_pos(cursor);
    }

    pub fn render_select(&mut self, ui: &Ui) {
        enum_combo(ui, "Decoration", self, ComboBoxFlags::empty());
        if ui.is_item_hovered() {
            ui.tooltip_text("Beware: many text decorations may negatively impact performance");
        }
    }
}
