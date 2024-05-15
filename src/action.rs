use crate::{
    context::EditState,
    elements::{Dnd, Element},
};
use std::ops;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[must_use]
pub enum Action {
    #[default]
    None,
    Cut,
    Copy,
    Up,
    Down,
    Delete,
    Drag,
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
            Self::None => {}
            Self::Cut => {
                let child = children.remove(index);
                log::debug!("Cut child {index} {}", child.kind.as_ref());
                edit.set_clipboard(child);
            }
            Self::Copy => {
                let child = children[index].clone();
                log::debug!("Copy child {index} {}", child.kind.as_ref());
                edit.set_clipboard(child);
            }
            Self::Up => {
                log::debug!("Move child up {index} {}", children[index].kind.as_ref());
                if index > 0 {
                    children.swap(index, index - 1);
                }
            }
            Self::Down => {
                log::debug!("Move child down {index} {}", children[index].kind.as_ref());
                if index < children.len() - 1 {
                    children.swap(index, index + 1);
                }
            }
            Self::Delete => {
                let child = children.remove(index);
                log::debug!("Delete child {index} {}", child.kind.as_ref());
            }
            Self::Drag => {
                let child = children.remove(index); // TODO: remove at end of drag?
                log::debug!("Drag child {index} {}", child.kind.as_ref());
                Dnd::set_dragging(child);
            }
        }
    }
}

impl ops::BitOrAssign for Action {
    fn bitor_assign(&mut self, rhs: Self) {
        self.or(rhs)
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

impl ops::BitOrAssign for ChildAction {
    fn bitor_assign(&mut self, rhs: Self) {
        self.or(rhs.index, rhs.kind)
    }
}
