use super::VisitMut;
use crate::{
    elements::{ElementAnchor, Bar, Common, Element, IconElement, IconList, Pack, Text},
    render::ComponentWise,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct Resizer {
    factor: f32,
}

impl Resizer {
    pub fn resize_pack(pack: &mut Pack, factor: f32) {
        log::debug!("Resize Pack {:?} with factor {}", pack.common.name, factor);
        Self { factor }.visit_elements(&mut pack.elements);
    }

    pub fn resize_element(element: &mut Element, factor: f32) {
        log::debug!(
            "Resize {} {:?} with factor {}",
            element.kind.as_ref(),
            element.common.name,
            factor
        );
        Self { factor }.visit_element_type(&mut element.kind);
    }
}

impl VisitMut for Resizer {
    fn visit_common(&mut self, common: &mut Common) {
        if common.anchor == ElementAnchor::Parent {
            common.pos = common.pos.mul_scalar(self.factor);
        }
    }

    fn visit_icon_list(&mut self, list: &mut IconList) {
        list.size = list.size.mul_scalar(self.factor);
        list.pad *= self.factor;
    }

    fn visit_icon(&mut self, icon: &mut IconElement) {
        icon.size = icon.size.mul_scalar(self.factor);
    }

    fn visit_text(&mut self, text: &mut Text) {
        text.props.base.scale *= self.factor;
        for condition in &mut text.props.conditions {
            if let Some(scale) = &mut condition.properties.scale {
                *scale *= self.factor;
            }
        }
    }

    fn visit_bar(&mut self, bar: &mut Bar) {
        bar.size = bar.size.mul_scalar(self.factor);
    }
}
