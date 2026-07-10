use super::VisitMut;
use crate::{
    elements::{Bar, Icon, Text},
    trigger::FilterTrigger,
};

#[derive(Debug, Clone, Copy)]
pub struct Loader;

impl Loader {
    pub fn new() -> Self {
        Self
    }
}

impl VisitMut for Loader {
    fn visit_icon(&mut self, icon: &mut Icon) {
        icon.load()
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
