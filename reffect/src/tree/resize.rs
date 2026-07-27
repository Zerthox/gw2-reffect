use super::VisitMut;
use crate::{
    elements::{Bar, Common, Element, ElementAnchor, IconElement, IconList, Pack, Text},
    math::ComponentWise,
    render::{helper, input_percent},
};
use nexus::imgui::Ui;

#[derive(Debug, Clone, Copy)]
pub struct Resizer {
    factor: f32,
    round: bool,
}

impl Default for Resizer {
    fn default() -> Self {
        Self {
            factor: 1.0,
            round: false,
        }
    }
}

impl Resizer {
    pub fn resize_pack(mut self, pack: &mut Pack) {
        log::debug!("Resize Pack {:?} with {self:?}", pack.common.name);
        self.visit_elements(&mut pack.elements);
    }

    pub fn resize_element(mut self, element: &mut Element) {
        log::debug!(
            "Resize {} {:?} with {self:?}",
            element.kind.as_ref(),
            element.common.name,
        );
        self.visit_element_type(&mut element.kind);
    }

    pub fn render_inputs(&mut self, ui: &Ui) {
        input_percent("Scale", &mut self.factor);
        ui.checkbox("Pixel perfect", &mut self.round);
        helper(ui, || {
            ui.text("Round to the nearest whole pixel");
            ui.text("Makes the operation potentially irreversible");
        });
    }

    fn scale_value(&self, value: &mut f32) {
        let scaled = *value * self.factor;
        *value = if self.round {
            scaled.round_ties_even()
        } else {
            scaled
        };
    }

    fn scale_value_no_round(&self, value: &mut f32) {
        *value *= self.factor;
    }

    fn scale_vec(&self, vec: &mut impl ComponentWise<f32>) {
        let scaled = vec.mul_scalar(self.factor);
        *vec = if self.round {
            scaled.round_ties_even()
        } else {
            scaled
        };
    }
}

impl VisitMut for Resizer {
    fn visit_common(&mut self, common: &mut Common) {
        if common.anchor == ElementAnchor::Parent {
            self.scale_vec(&mut common.pos);
        }
        self.visit_children_of(common);
    }

    fn visit_icon_list(&mut self, list: &mut IconList) {
        self.scale_vec(&mut list.size);
        self.scale_value(&mut list.pad);
        self.visit_children_of(list);
    }

    fn visit_icon_element(&mut self, icon: &mut IconElement) {
        self.scale_vec(&mut icon.size);
        self.visit_children_of(icon);
    }

    fn visit_text(&mut self, text: &mut Text) {
        self.scale_value_no_round(&mut text.props.base.scale);
        for condition in &mut text.props.conditions {
            if let Some(scale) = &mut condition.properties.scale {
                self.scale_value_no_round(scale);
            }
        }
    }

    fn visit_bar(&mut self, bar: &mut Bar) {
        self.scale_vec(&mut bar.size)
    }
}
