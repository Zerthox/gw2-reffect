use super::{Image, Line, Rect, Renderer, Text};
use crate::render::draw_text_bg;
use nexus::imgui::{TextureId, Ui};

#[derive(Debug)]
pub struct ImguiBgRenderer<'ui> {
    ui: &'ui Ui<'ui>,
}

impl Renderer for ImguiBgRenderer<'_> {
    fn init(&mut self) {}

    fn deinit(&mut self) {}

    fn line(&mut self, line: &Line) {
        let Line {
            start,
            end,
            color,
            thickness,
        } = *line;
        self.ui
            .get_background_draw_list()
            .add_line(start, end, color)
            .thickness(thickness)
            .build();
    }

    fn rect(&mut self, rect: &Rect) {
        let Rect {
            start,
            end,
            color,
            filled,
            rounding,
        } = *rect;
        self.ui
            .get_background_draw_list()
            .add_rect(start, end, color)
            .filled(filled)
            .rounding(rounding)
            .build();
    }

    fn image(&mut self, img: &Image) {
        let Image {
            texture,
            start,
            end,
            uv_min,
            uv_max,
            tint,
            rounding,
        } = *img;
        self.ui
            .get_background_draw_list()
            .add_image_rounded(TextureId::new(texture), start, end, rounding)
            .col(tint)
            .uv_min(uv_min)
            .uv_max(uv_max)
            .build();
    }

    fn text(&mut self, text: &Text) {
        let Text {
            text,
            pos,
            font_size,
            color,
        } = *text;
        draw_text_bg(self.ui, text, pos, font_size, color);
    }
}
