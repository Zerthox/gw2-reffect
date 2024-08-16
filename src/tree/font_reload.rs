use super::VisitMut;
use crate::elements::Text;

#[derive(Debug, Default, Clone, Copy)]
pub struct FontReloader;

impl VisitMut for FontReloader {
    fn visit_text(&mut self, text: &mut Text) {
        text.load_font();
    }
}
