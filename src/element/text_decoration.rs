use crate::component_wise::ComponentWise;
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

impl TextDecoration {
    pub fn render(&self, ui: &Ui, text: impl AsRef<str>, color: [f32; 4]) {
        let text = text.as_ref();
        let cursor = ui.cursor_screen_pos();
        let draw_list = ui.get_window_draw_list();
        match self {
            TextDecoration::None => {}
            TextDecoration::Shadow => {
                draw_list.add_text(cursor.add([1.0, 0.0]), color, text);
                draw_list.add_text(cursor.add([1.0, 1.0]), color, text);
            }
            TextDecoration::Outline => {
                for offset in [[-1.0, -1.0], [-1.0, 1.0], [1.0, -1.0], [1.0, 1.0]] {
                    draw_list.add_text(cursor.add(offset), color, text);
                }
            }
        }
    }
}
