use crate::{context::EditState, elements::Element};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[must_use]
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
            Action::Cut => {
                let child = children.remove(index);
                log::debug!("Cut child {index} {}", child.kind.as_ref());
                edit.set_clipboard(child);
            }
            Action::Copy => {
                let child = children[index].clone();
                log::debug!("Copy child {index} {}", child.kind.as_ref());
                edit.set_clipboard(child);
            }
            Action::Delete => {
                let child = children.remove(index);
                log::debug!("Delete child {index} {}", child.kind.as_ref());
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
