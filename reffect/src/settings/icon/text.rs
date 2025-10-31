use crate::{
    colors::{self, with_alpha},
    elements::{Anchor, text::TextDecoration},
    render::{
        ComponentWise, draw_text_bg, enum_combo, input_color_alpha, input_percent, input_pos,
    },
};
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextSettings {
    pub scale: f32,
    pub anchor: Anchor,
    pub offset: [f32; 2],
    pub decoration: TextDecoration,
    pub color: [f32; 4],
}

impl TextSettings {
    pub fn font_scale(&self, ui: &Ui, font_size: f32) -> f32 {
        let font_size = self.scale * font_size;
        font_size / ui.current_font_size()
    }

    pub fn size(&self, ui: &Ui, font_size: f32, text: impl AsRef<str>) -> [f32; 2] {
        let font_scale = self.font_scale(ui, font_size);
        ui.calc_text_size(text).mul_scalar(font_scale)
    }

    pub fn pos(
        &self,
        ui: &Ui,
        pos: [f32; 2],
        parent_size: [f32; 2],
        font_size: f32,
        text: impl AsRef<str>,
    ) -> [f32; 2] {
        let text_size = self.size(ui, font_size, text);
        let anchor_offset = self.anchor.pos(parent_size);
        let align_offset = self.anchor.align(text_size);
        pos.add(anchor_offset).add(align_offset).add(self.offset)
    }

    pub fn render(
        &self,
        ui: &Ui,
        pos: [f32; 2],
        parent_size: [f32; 2],
        color: [f32; 4],
        font_size: f32,
        text: impl AsRef<str>,
    ) {
        let text = text.as_ref();
        let [_, _, _, alpha] = color;
        let font_scale = self.font_scale(ui, font_size);
        let pos = self.pos(ui, pos, parent_size, font_size, text);

        let decoration_color = with_alpha(colors::BLACK, alpha);
        self.decoration
            .render(ui, text, pos, font_scale, decoration_color);
        draw_text_bg(ui, text, pos, font_scale, color);
    }

    pub fn render_options(&mut self, ui: &Ui) {
        let Self {
            scale,
            anchor,
            offset,
            decoration,
            color,
        } = self;

        input_percent("Scale", scale);
        enum_combo(ui, "Anchor", anchor, ComboBoxFlags::empty());
        input_pos(offset);
        enum_combo(ui, "Decoration", decoration, ComboBoxFlags::empty());
        input_color_alpha(ui, "Color", color);
    }
}
