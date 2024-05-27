use super::VisitMut;
use crate::elements::{IconElement, IconGrid};

#[derive(Debug, Default, Clone, Copy)]
pub struct Loader;

impl VisitMut for Loader {
    fn visit_icon_grid(&mut self, el: &mut IconGrid) {
        for icon in &mut el.icons {
            icon.inner.load();
        }
    }

    fn visit_icon(&mut self, el: &mut IconElement) {
        el.icon.load()
    }
}
