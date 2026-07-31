use super::VisitMut;
use crate::{
    elements::{Bar, Common, Icon, Text, list::ListIcon},
    trigger::FilterTrigger,
};

#[derive(Debug, Clone, Copy)]
pub struct Loader;

impl Loader {
    pub const fn new() -> Self {
        Self
    }
}

impl Default for Loader {
    fn default() -> Self {
        Self::new()
    }
}

impl VisitMut for Loader {
    fn visit_common(&mut self, common: &mut Common) {
        common.load();
        self.visit_children_of(common);
    }

    fn visit_filter_trigger(&mut self, trigger: &mut FilterTrigger) {
        trigger.load();
    }

    fn visit_list_icon(&mut self, list_icon: &mut ListIcon) {
        list_icon.load();
        self.visit_children_of(list_icon);
    }

    fn visit_icon(&mut self, icon: &mut Icon) {
        icon.load()
    }

    fn visit_text(&mut self, text: &mut Text) {
        text.load();
    }

    fn visit_bar(&mut self, bar: &mut Bar) {
        bar.load();
    }
}
