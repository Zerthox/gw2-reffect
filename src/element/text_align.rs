use nexus::imgui::{Selectable, Ui};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, IntoEnumIterator};

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
    pub fn calc_pos(&self, ui: &Ui, text: impl AsRef<str>) -> [f32; 2] {
        let [text_x, _] = ui.calc_text_size(text);
        match self {
            TextAlign::Left => [0.0, 0.0],
            TextAlign::Center => [-0.5 * text_x, 0.0],
            TextAlign::Right => [-text_x, 0.0],
        }
    }

    pub fn render_combo(&mut self, ui: &Ui) {
        if let Some(_token) = ui.begin_combo("Text position", &self) {
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
