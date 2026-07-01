use crate::{
    elements::{
        Bar, Common, Element, ElementType, Group, Icon, IconElement, IconList, Pack, Text,
        list::ListIcon,
    },
    trigger::{FilterTrigger, ProgressTrigger},
};

pub trait VisitMut: Sized {
    fn visit_children_of(&mut self, node: &mut impl Walk) {
        node.walk_mut(self);
    }

    fn visit_pack(&mut self, pack: &mut Pack) {
        self.visit_children_of(pack);
    }

    fn visit_element(&mut self, element: &mut Element) {
        self.visit_children_of(element);
    }

    fn visit_common(&mut self, common: &mut Common) {
        self.visit_children_of(common);
    }

    fn visit_progress_trigger(&mut self, _trigger: &mut ProgressTrigger) {}

    fn visit_filter_trigger(&mut self, _trigger: &mut FilterTrigger) {}

    fn visit_element_type(&mut self, element: &mut ElementType) {
        self.visit_children_of(element);
    }

    fn visit_group(&mut self, group: &mut Group) {
        self.visit_children_of(group);
    }

    fn visit_icon_list(&mut self, list: &mut IconList) {
        self.visit_children_of(list);
    }

    fn visit_list_icon(&mut self, list_icon: &mut ListIcon) {
        self.visit_children_of(list_icon);
    }

    fn visit_icon_element(&mut self, icon_element: &mut IconElement) {
        self.visit_children_of(icon_element);
    }

    fn visit_icon(&mut self, _icon: &mut Icon) {}

    fn visit_text(&mut self, _text: &mut Text) {}

    fn visit_bar(&mut self, _bar: &mut Bar) {}

    fn visit_packs<'i>(&mut self, packs: impl IntoIterator<Item = &'i mut Pack>) {
        for pack in packs {
            self.visit_pack(pack);
        }
    }

    fn visit_elements<'i>(&mut self, elements: impl IntoIterator<Item = &'i mut Element>) {
        for element in elements {
            self.visit_element(element);
        }
    }
}

/// Walkable tree node.
pub trait Walk {
    fn walk_mut(&mut self, visitor: &mut impl VisitMut);
}

impl Walk for Pack {
    fn walk_mut(&mut self, visitor: &mut impl VisitMut) {
        visitor.visit_common(&mut self.common);
        visitor.visit_elements(&mut self.elements);
    }
}

impl Walk for Element {
    fn walk_mut(&mut self, visitor: &mut impl VisitMut) {
        visitor.visit_common(&mut self.common);
        visitor.visit_element_type(&mut self.kind);
    }
}

impl Walk for Common {
    fn walk_mut(&mut self, visitor: &mut impl VisitMut) {
        visitor.visit_filter_trigger(&mut self.filter);
        visitor.visit_progress_trigger(&mut self.trigger);
    }
}

impl Walk for ElementType {
    fn walk_mut(&mut self, visitor: &mut impl VisitMut) {
        match self {
            Self::Group(group) => visitor.visit_group(group),
            Self::Icon(icon) => visitor.visit_icon_element(icon),
            Self::IconList(list) => visitor.visit_icon_list(list),
            Self::Text(text) => visitor.visit_text(text),
            Self::Bar(bar) => visitor.visit_bar(bar),
        }
    }
}

impl Walk for Group {
    fn walk_mut(&mut self, visitor: &mut impl VisitMut) {
        visitor.visit_elements(&mut self.members);
    }
}

impl Walk for IconList {
    fn walk_mut(&mut self, visitor: &mut impl VisitMut) {
        for icon in &mut self.icons {
            visitor.visit_list_icon(icon);
        }
    }
}

impl Walk for ListIcon {
    fn walk_mut(&mut self, visitor: &mut impl VisitMut) {
        visitor.visit_icon(&mut self.icon);
        visitor.visit_filter_trigger(&mut self.filter);
        visitor.visit_progress_trigger(&mut self.trigger);
    }
}

impl Walk for IconElement {
    fn walk_mut(&mut self, visitor: &mut impl VisitMut) {
        visitor.visit_icon(&mut self.icon);
    }
}
