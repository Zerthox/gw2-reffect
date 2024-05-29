use nexus::imgui::{Selectable, Ui};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, IntoEnumIterator};

// TODO: vertical align?

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
pub enum TextAlign {
    Left,
    Center,
    Right,
}

impl TextAlign {
    pub fn calc_pos(&self, ui: &Ui, text: impl AsRef<str>, font_scale: f32) -> [f32; 2] {
        let [text_x, _] = ui.calc_text_size(text);
        let width = font_scale * text_x;
        let line_height = ui.text_line_height();
        let offset_y = -0.5 * font_scale * line_height;
        match self {
            TextAlign::Left => [0.0, offset_y],
            TextAlign::Center => [-0.5 * width, offset_y],
            TextAlign::Right => [-width, offset_y],
        }
    }

    pub fn render_combo(&mut self, ui: &Ui) {
        if let Some(_token) = ui.begin_combo("Align", &self) {
            for entry in Self::iter() {
                let selected = entry == *self;
                if Selectable::new(&entry).selected(selected).build(ui) {
                    *self = entry;
                }
                if selected {
                    ui.set_item_default_focus();
                }
            }
        }
    }
}
