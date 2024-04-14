use crate::{component_wise::ComponentWise, render_util::impl_static_variants};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter};

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
    Serialize,
    Deserialize,
)]
pub enum TextDecoration {
    None,
    Shadow,
    Outline,
}

impl_static_variants!(TextDecoration);

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
}
