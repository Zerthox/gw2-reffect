use crate::colors::Color;

mod imgui;

#[derive(Debug, Clone)]
pub struct Line {
    pub start: [f32; 2],
    pub end: [f32; 2],
    pub color: Color,
    pub thickness: f32,
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub start: [f32; 2],
    pub end: [f32; 2],
    pub color: Color,
    pub filled: bool,
    pub rounding: f32,
}

#[derive(Debug, Clone)]
pub struct Image {
    pub texture: usize,
    pub start: [f32; 2],
    pub end: [f32; 2],
    pub uv_min: [f32; 2],
    pub uv_max: [f32; 2],
    pub tint: Color,
    pub rounding: f32,
}

#[derive(Debug, Clone)]
pub struct Text<'a> {
    pub text: &'a str,
    pub pos: [f32; 2],
    pub font_size: f32,
    pub color: Color,
}

pub trait Renderer {
    fn init(&mut self);

    fn deinit(&mut self);

    fn line(&mut self, line: &Line);

    fn rect(&mut self, rect: &Rect);

    fn image(&mut self, img: &Image);

    fn text(&mut self, text: &Text);
}
