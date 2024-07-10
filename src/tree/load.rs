use super::VisitMut;
use crate::{
    elements::{IconElement, IconList},
    trigger::MetaTrigger,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct Loader;

impl VisitMut for Loader {
    fn visit_icon_list(&mut self, el: &mut IconList) {
        for icon in &mut el.icons {
            icon.inner.load();
        }
    }

    fn visit_icon(&mut self, el: &mut IconElement) {
        el.icon.load()
    }

    fn visit_meta_trigger(&mut self, trigger: &mut MetaTrigger) {
        trigger.load();
    }
}
