use crate::{context::EditState, elements::Element};

#[derive(Debug, Default, Clone)]
pub enum Action {
    #[default]
    None,
    Cut,
    Copy,
    Delete,
}

impl Action {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn or(&mut self, other: Self) {
        if self.is_none() {
            *self = other;
        }
    }

    pub fn perform(self, edit: &mut EditState, children: &mut Vec<Element>, index: usize) {
        match self {
            Action::None => {}
            Action::Cut => edit.set_clipboard(children.remove(index)),
            Action::Copy => edit.set_clipboard(children[index].clone()),
            Action::Delete => {
                children.remove(index);
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ChildAction {
    pub index: usize,
    pub kind: Action,
}

impl ChildAction {
    pub const fn new() -> Self {
        Self {
            index: 0,
            kind: Action::None,
        }
    }

    pub fn or(&mut self, index: usize, other: Action) {
        if self.kind.is_none() {
            self.kind = other;
            self.index = index;
        }
    }

    pub fn perform(self, edit: &mut EditState, children: &mut Vec<Element>) {
        self.kind.perform(edit, children, self.index)
    }
}
