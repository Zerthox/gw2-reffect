use crate::{
    elements::{Bar, Common, Element, ElementType, Group, IconElement, IconList, Pack, Text},
    trigger::FilterTrigger,
};

pub trait VisitMut {
    fn visit_packs(&mut self, packs: &mut [Pack]) {
        for pack in packs {
            self.visit_pack(pack);
        }
    }

    fn visit_pack(&mut self, pack: &mut Pack) {
        self.visit_common(&mut pack.common);
        self.visit_elements(&mut pack.elements);
    }

    fn visit_elements(&mut self, elements: &mut [Element]) {
        for element in elements {
            self.visit_element(element);
        }
    }

    fn visit_element(&mut self, element: &mut Element) {
        self.visit_common(&mut element.common);
        self.visit_filter_trigger(&mut element.filter);
        self.visit_element_type(&mut element.kind);
    }

    fn visit_element_type(&mut self, element_type: &mut ElementType) {
        match element_type {
            ElementType::Group(group) => {
                self.visit_group(group);
                self.visit_elements(&mut group.members);
            }
            ElementType::Icon(icon) => self.visit_icon(icon),
            ElementType::IconList(list) => {
                self.visit_icon_list(list);
                for icon in &mut list.icons {
                    self.visit_filter_trigger(&mut icon.filter);
                }
            }
            ElementType::Text(text) => self.visit_text(text),
            ElementType::Bar(bar) => self.visit_bar(bar),
        }
    }

    fn visit_common(&mut self, _common: &mut Common) {}

    fn visit_filter_trigger(&mut self, _trigger: &mut FilterTrigger) {}

    fn visit_group(&mut self, _group: &mut Group) {}

    fn visit_icon_list(&mut self, _list: &mut IconList) {}

    fn visit_icon(&mut self, _icon: &mut IconElement) {}

    fn visit_text(&mut self, _text: &mut Text) {}

    fn visit_bar(&mut self, _bar: &mut Bar) {}
}
