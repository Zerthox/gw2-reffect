use super::VisitMut;
use crate::{
    elements::{IconElement, IconList, Text},
    trigger::FilterTrigger,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct Loader;

impl VisitMut for Loader {
    fn visit_icon_list(&mut self, el: &mut IconList) {
        for icon in &mut el.icons {
            icon.icon.load();
        }
    }

    fn visit_icon(&mut self, el: &mut IconElement) {
        el.icon.load()
    }

    fn visit_text(&mut self, text: &mut Text) {
        text.load();
    }

    fn visit_filter_trigger(&mut self, trigger: &mut FilterTrigger) {
        trigger.load();
    }
}
