use nexus::imgui::{Selectable, Ui};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, IntoEnumIterator};

/// Horizontal alignment.
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
pub enum AlignHorizontal {
    Left,
    Center,
    Right,
}

impl AlignHorizontal {
    pub fn text_offset(&self, ui: &Ui, text: impl AsRef<str>, font_scale: f32) -> [f32; 2] {
        let [text_x, _] = ui.calc_text_size(text);
        let width = font_scale * text_x;
        let offset_x = self.item_offset_x(width);
        let line_height = ui.text_line_height();
        let offset_y = -0.5 * font_scale * line_height;
        [offset_x, offset_y]
    }

    pub fn item_offset_x(&self, width: f32) -> f32 {
        match self {
            Self::Left => 0.0,
            Self::Center => -0.5 * width,
            Self::Right => -width,
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
