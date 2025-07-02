use crate::{
    clipboard::Clipboard,
    elements::{Dnd, Element},
};
use std::ops;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[must_use]
pub enum ElementAction {
    #[default]
    None,
    Cut,
    Copy,
    Duplicate,
    Up,
    Down,
    Delete,
    Drag,
}

impl ElementAction {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn or(&mut self, other: Self) {
        if self.is_none() {
            *self = other;
        }
    }

    pub fn perform(self, children: &mut Vec<Element>, index: usize) {
        match self {
            Self::None => {}
            Self::Cut => {
                let child = children.remove(index);
                log::debug!("Cut child {index} {}", child.kind.as_ref());
                Clipboard::set(child);
            }
            Self::Copy => {
                let child = children[index].clone();
                log::debug!("Copy child {index} {}", child.kind.as_ref());
                Clipboard::set(child);
            }
            Self::Duplicate => {
                let child = children[index].clone();
                log::debug!("Duplicate child {index} {}", child.kind.as_ref());
                children.insert(index + 1, child);
            }
            Self::Up => {
                log::debug!("Move child up {index} {}", children[index].kind.as_ref());
                if index == 0 {
                    let first = children.remove(0);
                    children.push(first);
                } else {
                    children.swap(index, index - 1);
                }
            }
            Self::Down => {
                log::debug!("Move child down {index} {}", children[index].kind.as_ref());
                if index == children.len() - 1 {
                    let last = children.pop().expect("element action down with empty vec");
                    children.insert(0, last);
                } else {
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

impl ops::BitOrAssign for ElementAction {
    fn bitor_assign(&mut self, rhs: Self) {
        self.or(rhs)
    }
}

#[derive(Debug, Default, Clone)]
pub struct ChildElementAction {
    pub index: usize,
    pub kind: ElementAction,
}

impl ChildElementAction {
    pub const fn new() -> Self {
        Self {
            index: 0,
            kind: ElementAction::None,
        }
    }

    pub fn or(&mut self, index: usize, other: ElementAction) {
        if self.kind.is_none() {
            self.kind = other;
            self.index = index;
        }
    }

    pub fn perform(self, children: &mut Vec<Element>) {
        self.kind.perform(children, self.index)
    }
}

impl ops::BitOrAssign for ChildElementAction {
    fn bitor_assign(&mut self, rhs: Self) {
        self.or(rhs.index, rhs.kind)
    }
}
