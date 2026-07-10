use super::VisitMut;
use crate::{elements::Text, render::Io};

#[derive(Debug, Clone, Copy)]
pub struct FontLoader {
    io: Io,
}

impl FontLoader {
    pub fn new(io: Io) -> Self {
        Self { io }
    }
}

impl VisitMut for FontLoader {
    fn visit_text(&mut self, text: &mut Text) {
        text.font.load(self.io);
    }
}
