mod load;

pub use self::load::*;

use crate::elements::{Element, ElementType, Group, IconElement, IconGrid, Text};

pub trait VisitMut {
    fn visit_elements(&mut self, elements: &mut [Element]) {
        for element in elements {
            self.visit_element(element);
        }
    }

    fn visit_element(&mut self, element: &mut Element) {
        match &mut element.kind {
            ElementType::Group(group) => {
                self.visit_group(group);
                self.visit_elements(&mut group.members);
            }
            ElementType::IconGrid(grid) => {
                self.visit_icon_grid(grid);
            }
            ElementType::Icon(icon) => self.visit_icon(icon),
            ElementType::Text(text) => self.visit_text(text),
        }
    }

    fn visit_group(&mut self, _group: &mut Group) {}

    fn visit_icon_grid(&mut self, _grid: &mut IconGrid) {}

    fn visit_icon(&mut self, _icon: &mut IconElement) {}

    fn visit_text(&mut self, _text: &mut Text) {}
}
