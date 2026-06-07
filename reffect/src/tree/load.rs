use super::VisitMut;
use crate::{
    elements::{Bar, IconElement, IconList, Text},
    trigger::FilterTrigger,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct Loader;

impl VisitMut for Loader {
    fn visit_icon_list(&mut self, list: &mut IconList) {
        list.load();
    }

    fn visit_icon(&mut self, icon: &mut IconElement) {
        icon.icon.load()
    }

    fn visit_text(&mut self, text: &mut Text) {
        text.load();
    }

    fn visit_bar(&mut self, bar: &mut Bar) {
        bar.load();
    }

    fn visit_filter_trigger(&mut self, trigger: &mut FilterTrigger) {
        trigger.load();
    }
}
